use crate::POLYBAR_CFG;

use crate::common::{
	constraint::{Constraint, Constraints},
	groups, name, outputs, polybar, this_command_abs, workspaces,
};

pub fn exec(_: Vec<String>) {
	let show_hidden = groups::show_hidden_enabled();
	let focused_output = outputs::focused();

	let mut show_constraints = Constraints::new();
	show_constraints.add(Constraint::NoGroup);
	show_constraints.add(Constraint::AllowUrgent);

	if !show_hidden {
		show_constraints.add(Constraint::Group);
	}
	if POLYBAR_CFG.i3_wsman.pin_workspaces == true {
		show_constraints.add(Constraint::Output);
		show_constraints.output = focused_output.clone();
	}

	let visible_workspaces = workspaces::visible();
	let mut workspaces = workspaces::get(show_constraints, false);

	for visible_ws in visible_workspaces {
		if !workspaces.contains(&visible_ws) && visible_ws.output == focused_output {
			workspaces.push(visible_ws);
		}
	}

	workspaces.sort_by(|w1, w2| w1.num.cmp(&w2.num));

	let mut avail_constraints = Constraints::new();
	if POLYBAR_CFG.i3_wsman.pin_workspaces == true {
		avail_constraints.add(Constraint::Output);
		avail_constraints.output = focused_output.clone();
	}
	let groups = groups::available(avail_constraints);

	let active_groups = groups::active(focused_output.clone());
	let showing_all = active_groups.len() == 0 || groups == active_groups;
	let mut cur_output = "".to_string();
	for ws in workspaces {
		if cur_output.len() > 0 && cur_output != ws.output {
			print!("  ");
		}
		cur_output = ws.output.to_owned();

		let mut ws_label_btn = polybar::Label::new(polybar::defaults::WS_LABEL, 2, 0);
		ws_label_btn.font = Some(3);

		let cmd = this_command_abs() + " polybar goto " + ws.num.to_string().as_ref();
		ws_label_btn.set_action(polybar::LEFT_CLICK, &cmd);

		let ws_group = name::group(ws.name.as_str());
		if ws_group == "" || showing_all || active_groups.contains(&ws_group) {
			if ws.focused {
				ws_label_btn
					.set_colors(polybar::defaults::FOCUSED_FG, polybar::defaults::FOCUSED_BG);
				ws_label_btn.label = polybar::defaults::FOCUSED_WS_LABEL.to_string();
			} else if ws.urgent {
				ws_label_btn.set_colors(polybar::defaults::URGENT_FG, polybar::defaults::URGENT_BG);
				ws_label_btn.label = polybar::defaults::URGENT_WS_LABEL.to_string();
			} else if ws.visible {
				ws_label_btn
					.set_colors(polybar::defaults::VISIBLE_FG, polybar::defaults::VISIBLE_BG);
				ws_label_btn.label = polybar::defaults::VISIBLE_WS_LABEL.to_string();
			} else {
				ws_label_btn.set_colors(polybar::defaults::FG, polybar::defaults::BG);
			}

			if ws_group == "" && !showing_all {
				if ws.focused {
					ws_label_btn.label = polybar::defaults::HIDDEN_FOCUSED_WS_LABEL.to_string();
				} else if ws.urgent {
					ws_label_btn.label = polybar::defaults::HIDDEN_URGENT_WS_LABEL.to_string();
				} else if ws.visible {
					ws_label_btn.label = polybar::defaults::HIDDEN_VISIBLE_WS_LABEL.to_string();
				} else {
					ws_label_btn.label = polybar::defaults::HIDDEN_WS_LABEL.to_string();
				}
			}
		} else {
			if ws.focused {
				ws_label_btn.set_colors(
					polybar::defaults::HIDDEN_FOCUSED_FG,
					polybar::defaults::HIDDEN_FOCUSED_BG,
				);
				ws_label_btn.label = polybar::defaults::HIDDEN_FOCUSED_WS_LABEL.to_string();
			} else if ws.urgent {
				ws_label_btn.set_colors(
					polybar::defaults::HIDDEN_URGENT_FG,
					polybar::defaults::HIDDEN_URGENT_BG,
				);
				ws_label_btn.label = polybar::defaults::HIDDEN_URGENT_WS_LABEL.to_string();
			} else if ws.visible {
				ws_label_btn.set_colors(
					polybar::defaults::HIDDEN_VISIBLE_FG,
					polybar::defaults::HIDDEN_VISIBLE_BG,
				);
				ws_label_btn.label = polybar::defaults::HIDDEN_VISIBLE_WS_LABEL.to_string();
			} else {
				ws_label_btn.set_colors(polybar::defaults::HIDDEN_FG, polybar::defaults::HIDDEN_BG);
				ws_label_btn.label = polybar::defaults::HIDDEN_WS_LABEL.to_string();
			}
		}

		print!("{}", ws_label_btn);
	}
}
