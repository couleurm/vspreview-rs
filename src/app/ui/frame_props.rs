use super::{egui, egui::RichText, epi, VSPreviewer, STATE_LABEL_COLOR};

pub struct UiFrameProps {}

impl UiFrameProps {
    pub fn ui(pv: &mut VSPreviewer, frame: &epi::Frame, ui: &mut egui::Ui) {
        let output = pv.outputs.get(&pv.state.cur_output).unwrap();
        let mut props = None;

        if let Some(promise) = &output.frame_promise {
            if let Some(pf) = promise.ready() {
                if let Ok(pf) = &pf.read() {
                    props = Some(pf.vsframe.props.clone())
                }
            }
        }

        // Overwrite from original if available
        if let Some(promise) = &output.original_props_promise {
            if let Some(Some(p)) = promise.ready() {
                props = Some(p.clone());
            }
        }

        if let Some(props) = props {
            let header = RichText::new("Frame props").color(STATE_LABEL_COLOR);

            egui::CollapsingHeader::new(header).show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;

                egui::Grid::new("props_grid")
                    .num_columns(2)
                    .spacing([8.0, -2.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Frame type").color(STATE_LABEL_COLOR));
                        ui.label(props.frame_type);
                        ui.end_row();

                        ui.label(RichText::new("Color range").color(STATE_LABEL_COLOR));
                        ui.label(props.color_range.to_string());
                        ui.end_row();

                        ui.label(RichText::new("Chroma location").color(STATE_LABEL_COLOR));
                        ui.label(props.chroma_location.to_string());
                        ui.end_row();

                        ui.label(RichText::new("Primaries").color(STATE_LABEL_COLOR));
                        ui.label(props.primaries.to_string());
                        ui.end_row();

                        ui.label(RichText::new("Matrix").color(STATE_LABEL_COLOR));
                        ui.label(props.matrix.to_string());
                        ui.end_row();

                        ui.label(RichText::new("Transfer").color(STATE_LABEL_COLOR));
                        ui.label(props.transfer.to_string());
                        ui.end_row();

                        if let Some(sc) = props.is_scenecut {
                            let (v, color) = crate::utils::icon_color_for_bool(sc);

                            ui.label(RichText::new("Scene cut").color(STATE_LABEL_COLOR));
                            ui.label(RichText::new(v).size(20.0).color(color));
                            ui.end_row();
                        }

                        if let Some(hdr10_meta) = props.hdr10_metadata {
                            ui.label(RichText::new("Mastering display").color(STATE_LABEL_COLOR));

                            let prim_label =
                                egui::Label::new(hdr10_meta.mastering_display.to_string())
                                    .sense(egui::Sense::click());
                            let mdcv_res = ui.add(prim_label);

                            ui.scope(|ui| {
                                if mdcv_res
                                    .on_hover_text("Click to copy x265 setting")
                                    .clicked()
                                {
                                    let arg = format!(
                                        "--master-display \"{}\"",
                                        hdr10_meta.mastering_display.x265_string()
                                    );
                                    ui.output().copied_text = arg;
                                }
                            });
                            ui.end_row();

                            if let (Some(maxcll), Some(maxfall)) =
                                (hdr10_meta.maxcll, hdr10_meta.maxfall)
                            {
                                ui.label(
                                    RichText::new("Content light level").color(STATE_LABEL_COLOR),
                                );

                                let cll_label = egui::Label::new(format!(
                                    "MaxCLL: {maxcll}, MaxFALL: {maxfall}"
                                ))
                                .sense(egui::Sense::click());
                                let cll_res = ui.add(cll_label);

                                ui.scope(|ui| {
                                    if cll_res
                                        .on_hover_text("Click to copy x265 setting")
                                        .clicked()
                                    {
                                        let arg = format!("--max-cll \"{},{}\"", maxcll, maxfall);
                                        ui.output().copied_text = arg;
                                    }
                                });
                                ui.end_row();
                            }
                        }

                        let (v, color) = crate::utils::icon_color_for_bool(props.is_dolbyvision);
                        ui.label(RichText::new("Dolby Vision").color(STATE_LABEL_COLOR));
                        ui.label(RichText::new(v).size(20.0).color(color));
                        ui.end_row();

                        if let Some(cambi) = props.cambi_score {
                            let rounded = egui::emath::round_to_decimals(cambi, 4);
                            ui.label(RichText::new("CAMBI score").color(STATE_LABEL_COLOR));
                            ui.label(rounded.to_string());
                            ui.end_row();
                        }

                        ui.label("");
                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                            let reload_btn = ui.button("Reload original props");

                            if reload_btn.clicked() {
                                pv.fetch_original_props(frame);
                            }
                        });
                        ui.end_row();
                    });
            });
        }
    }
}
