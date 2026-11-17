//! Original light and dark palette values. Compiled by the UI theme boundary.
use crate::theme::Tokens;
use egui::Color32;
impl Tokens {
    pub const fn light() -> Tokens {
        Tokens {
            app_bar: Color32::from_rgb(52, 56, 64),
            toolbar: Color32::from_rgb(246, 247, 249),
            panel: Color32::from_rgb(250, 251, 252),
            panel_header: Color32::from_rgb(236, 238, 242),
            border: Color32::from_rgb(208, 212, 220),
            text: Color32::from_rgb(38, 42, 50),
            text_dim: Color32::from_rgb(110, 116, 128),
            accent: Color32::from_rgb(38, 120, 218),
            accent_soft: Color32::from_rgb(214, 230, 250),
            sketch_accent: Color32::from_rgb(46, 150, 90),
            error: Color32::from_rgb(214, 54, 54),
            warning: Color32::from_rgb(222, 150, 30),
            icon: Color32::from_rgb(70, 78, 92),
            icon_fill: Color32::from_rgb(178, 196, 222),
            timeline: Color32::from_rgb(232, 235, 240),
            hover: Color32::from_rgb(226, 234, 246),
            sel_face: Color32::from_rgb(76, 127, 203),
            sel_edge: Color32::from_rgb(0, 127, 255),
            sel_edge_rim: Color32::from_rgb(127, 191, 255),
            hover_edge: Color32::from_rgb(0, 0, 0),
            hover_edge_halo: Color32::from_rgb(203, 203, 203),
            hover_face_lift: 40,
            sel_profile: Color32::from_rgb(26, 84, 167),
            sel_profile_edge: Color32::from_rgb(47, 109, 198),
            hover_profile: Color32::from_rgba_unmultiplied_const(75, 129, 253, 48),
            hover_profile_edge: Color32::from_rgb(75, 129, 253),
            sel_vertex: Color32::from_rgb(70, 110, 170),
            origin_plane: Color32::from_rgba_unmultiplied_const(249, 184, 134, 94),
            origin_plane_edge: Color32::from_rgba_unmultiplied_const(214, 140, 84, 200),
            origin_plane_hover: Color32::from_rgba_unmultiplied_const(120, 130, 230, 150),
            origin_point: Color32::from_rgb(214, 215, 218),
            axis_x: Color32::from_rgb(255, 0, 0),
            axis_y: Color32::from_rgb(0, 255, 0),
            axis_z: Color32::from_rgb(2, 2, 248),
            construction_plane: Color32::from_rgba_unmultiplied_const(249, 184, 134, 70),
            construction_geom: Color32::from_rgb(232, 140, 48),
            body: Color32::from_rgb(176, 186, 198),
            body_edge: Color32::from_rgb(40, 44, 52),
            manipulator: Color32::from_rgb(40, 110, 220),
            preview_add: Color32::from_rgb(112, 160, 226),
            preview_sketch: Color32::from_rgb(32, 190, 96),
            preview_cut: Color32::from_rgba_unmultiplied_const(232, 84, 44, 110),
            cube_face: Color32::from_rgb(236, 239, 244),
            cube_border: Color32::from_rgb(85, 94, 110),
            cube_text: Color32::from_rgb(38, 42, 50),
            cube_hover: Color32::from_rgb(190, 220, 252),
            viewport_top: Color32::from_rgb(240, 243, 247),
            viewport_bottom: Color32::from_rgb(203, 210, 221),
            grid: Color32::from_rgb(196, 202, 212),
            grid_major: Color32::from_rgb(170, 178, 190),
            body_selected: Color32::from_rgb(110, 170, 235),
            sketch: Color32::from_rgb(30, 90, 200),
            sketch_fixed: Color32::from_rgb(20, 20, 20),
            sketch_point: Color32::BLACK,
            sketch_locked: Color32::from_rgb(0, 140, 60),
            dimension: Color32::from_rgb(36, 40, 48),
            dimension_driven: Color32::from_rgb(120, 126, 138),
            sketch_point_free: Color32::from_rgb(30, 90, 200),
            section_cap: Color32::from_rgb(214, 128, 72),
            dialog_bg: Color32::from_rgb(250, 251, 252),
            dialog_border: Color32::from_rgb(208, 212, 220),
            chip: Color32::from_rgb(38, 120, 218),
            heading_bg: Color32::from_rgb(218, 223, 231),
            subheading_bg: Color32::from_rgb(234, 237, 242),
            profile_fill: Color32::from_rgba_unmultiplied_const(255, 196, 120, 70),
            glyph_bg: Color32::WHITE,
            glyph_edge: Color32::from_rgb(190, 110, 100),
            glyph_text: Color32::from_rgb(150, 40, 35),
            overlay: Color32::from_rgba_unmultiplied_const(255, 255, 255, 235),
            rubber_band: Color32::from_rgb(30, 120, 230),
            app_bar_text: Color32::from_rgb(230, 234, 242),
            timeline_item: Color32::WHITE,
            timeline_item_error: Color32::from_rgb(250, 220, 220),
            timeline_marker: Color32::from_rgb(70, 76, 88),
            field: Color32::WHITE,
            box_window: Color32::from_rgb(38, 120, 218),
            box_crossing: Color32::from_rgb(40, 160, 80),
        }
    }
    pub const fn dark() -> Tokens {
        let mut t = Tokens::light();
        // SolidCraft Burgundy & Gold palette.
        // Application chrome and surfaces.
        t.app_bar = Color32::from_rgb(45, 13, 19);
        t.toolbar = Color32::from_rgb(69, 24, 32);
        t.panel = Color32::from_rgb(82, 33, 41); // #522129
        t.panel_header = Color32::from_rgb(100, 43, 53);
        t.border = Color32::from_rgb(125, 63, 70);
        t.text = Color32::from_rgb(253, 243, 231); // #FDF3E7
        t.text_dim = Color32::from_rgba_unmultiplied(253, 243, 231, 199);
        t.accent = Color32::from_rgb(229, 168, 83); // #E5A853
        t.accent_soft = Color32::from_rgb(112, 69, 43);
        t.sketch_accent = Color32::from_rgb(201, 134, 59); // #C9863B
        t.error = Color32::from_rgb(239, 113, 105);
        t.warning = Color32::from_rgb(229, 168, 83);
        t.icon = Color32::from_rgb(242, 220, 195);
        t.icon_fill = Color32::from_rgb(128, 76, 59);
        t.timeline = Color32::from_rgb(59, 20, 26); // #3B141A
        t.hover = Color32::from_rgb(112, 55, 61);

        // Selection feedback uses the gold accent without changing axis semantics.
        t.sel_face = Color32::from_rgb(201, 134, 59);
        t.sel_edge = Color32::from_rgb(229, 168, 83);
        t.sel_edge_rim = Color32::from_rgb(253, 220, 160);
        t.hover_edge = Color32::from_rgb(253, 243, 231);
        t.hover_edge_halo = Color32::from_rgb(229, 168, 83);
        t.sel_profile = Color32::from_rgb(132, 78, 43);
        t.sel_profile_edge = Color32::from_rgb(229, 168, 83);
        t.hover_profile = Color32::from_rgba_unmultiplied_const(229, 168, 83, 48);
        t.hover_profile_edge = Color32::from_rgb(253, 220, 160);
        t.sel_vertex = Color32::from_rgb(229, 168, 83);

        // Origin planes and construction geometry.
        t.origin_plane = Color32::from_rgba_unmultiplied_const(229, 168, 83, 70);
        t.origin_plane_edge = Color32::from_rgba_unmultiplied_const(201, 134, 59, 200);
        t.origin_plane_hover = Color32::from_rgba_unmultiplied_const(229, 168, 83, 150);
        t.origin_point = Color32::from_rgb(253, 243, 231);
        t.construction_plane = Color32::from_rgba_unmultiplied_const(229, 168, 83, 48);
        t.construction_geom = Color32::from_rgb(201, 134, 59);

        // Neutral metallic model shading; keep XYZ axes in their conventional colours.
        t.body = Color32::from_rgb(174, 157, 145);
        t.body_edge = Color32::from_rgb(35, 15, 19);
        t.manipulator = Color32::from_rgb(229, 168, 83);
        t.preview_add = Color32::from_rgb(229, 168, 83);
        t.preview_sketch = Color32::from_rgb(201, 134, 59);
        t.preview_cut = Color32::from_rgba_unmultiplied_const(232, 84, 44, 110);
        t.cube_face = Color32::from_rgb(82, 33, 41);
        t.cube_border = Color32::from_rgb(201, 134, 59);
        t.cube_text = Color32::from_rgb(253, 243, 231);
        t.cube_hover = Color32::from_rgb(112, 69, 43);

        // Burgundy viewport with a restrained grid.
        t.viewport_top = Color32::from_rgb(59, 20, 26); // #3B141A
        t.viewport_bottom = Color32::from_rgb(59, 20, 26);
        t.grid = Color32::from_rgb(91, 43, 51);
        t.grid_major = Color32::from_rgb(119, 59, 65);
        t.body_selected = Color32::from_rgb(229, 168, 83);
        t.sketch = Color32::from_rgb(229, 168, 83);
        t.sketch_fixed = Color32::from_rgb(253, 243, 231);
        t.sketch_point = Color32::from_rgb(253, 243, 231);
        t.sketch_locked = Color32::from_rgb(201, 134, 59);
        t.dimension = Color32::from_rgb(253, 243, 231);
        t.dimension_driven = Color32::from_rgb(210, 183, 160);
        t.sketch_point_free = Color32::from_rgb(229, 168, 83);

        // Dialogs, chips, overlays, timeline and fields.
        t.dialog_bg = Color32::from_rgb(82, 33, 41);
        t.dialog_border = Color32::from_rgb(125, 63, 70);
        t.chip = Color32::from_rgb(201, 134, 59);
        t.heading_bg = Color32::from_rgb(59, 20, 26);
        t.subheading_bg = Color32::from_rgb(100, 43, 53);
        t.section_cap = Color32::from_rgb(201, 134, 59);
        t.profile_fill = Color32::from_rgba_unmultiplied_const(229, 168, 83, 42);
        t.glyph_bg = Color32::from_rgb(69, 24, 32);
        t.glyph_edge = Color32::from_rgb(229, 168, 83);
        t.glyph_text = Color32::from_rgb(253, 220, 160);
        t.overlay = Color32::from_rgba_unmultiplied_const(45, 13, 19, 238);
        t.rubber_band = Color32::from_rgb(229, 168, 83);
        t.app_bar_text = Color32::from_rgb(253, 243, 231);
        t.timeline_item = Color32::from_rgb(100, 43, 53);
        t.timeline_item_error = Color32::from_rgb(110, 50, 54);
        t.timeline_marker = Color32::from_rgb(229, 168, 83);
        t.field = Color32::from_rgb(59, 20, 26);
        t.box_window = Color32::from_rgb(229, 168, 83);
        t.box_crossing = Color32::from_rgb(201, 134, 59);
        t
    }
}
