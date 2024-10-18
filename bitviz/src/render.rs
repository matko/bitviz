use itertools::Itertools;
use svg::{
    node::element::{Definitions, Group, Rectangle, Style, Text, Use},
    Document,
};

pub struct RenderStyle {
    pub group_size: usize,
    pub line_size: usize,
    pub group_padding: usize,
    pub line_padding: usize,
    pub box_size: usize,
    pub style: Option<String>,
}

pub fn render_bits<I: Iterator<Item = bool>>(
    iter: I,
    style: &RenderStyle,
) -> (usize, usize, Group) {
    let mut page_group = Group::new();
    let mut max_line = 0;
    for (line_ix, line) in iter
        .chunks(style.group_size)
        .into_iter()
        .chunks(style.line_size)
        .into_iter()
        .enumerate()
    {
        max_line = line_ix;
        let mut line_group = Group::new().set(
            "transform",
            format!(
                "translate(0,{})",
                line_ix * (style.box_size + style.line_padding)
            ),
        );
        for (group_ix, group) in line.enumerate() {
            let mut bit_group = Group::new().set(
                "transform",
                format!(
                    "translate({},0)",
                    group_ix * (style.group_size * style.box_size + style.group_padding)
                ),
            );
            for (bit_ix, bit) in group.enumerate() {
                let bit_box = Use::new()
                    .set("href", if bit { "#bit1" } else { "#bit0" })
                    .set(
                        "transform",
                        format!("translate({},0)", bit_ix * style.box_size),
                    );
                bit_group = bit_group.add(bit_box);
            }
            line_group = line_group.add(bit_group);
        }

        page_group = page_group.add(line_group);
    }

    (
        style.line_size * style.group_size * style.box_size
            + (style.line_size - 1) * style.group_padding,
        (max_line + 1) * style.box_size + max_line * style.line_padding,
        page_group,
    )
}

fn render_bit_def(name: &str, bit: bool, style: &RenderStyle) -> Group {
    let suffix = if bit { "1" } else { "0" };
    Group::new()
        .set("id", name)
        .add(
            Rectangle::new()
                .set("class", format!("bit-box bit-box-{suffix}"))
                .set("height", style.box_size)
                .set("width", style.box_size)
                .set("x", 0)
                .set("y", 0)
                .set("stroke-width", style.box_size as f32 / 5.0),
        )
        .add(
            Text::new(suffix)
                .set("class", format!("bit-text bit-text-{suffix}"))
                .set("x", style.box_size / 2)
                .set("y", style.box_size / 2)
                .set("dy", "0.35em")
                .set("font-size", style.box_size),
        )
}

pub fn render_bits_doc<I: Iterator<Item = bool>>(iter: I, style: &RenderStyle) -> Document {
    let (dimx, dimy, bits) = render_bits(iter, style);
    let extra_style = style
        .style
        .as_ref()
        .map(|s| String::from_utf8(std::fs::read(s).unwrap()).unwrap())
        .unwrap_or("".to_string());
    Document::new()
        .set("width", dimx)
        .set("height", dimy)
        .add(Style::new(include_str!("default_style.css")))
        .add(Style::new(extra_style))
        .add(
            Definitions::new()
                .add(render_bit_def("bit0", false, style))
                .add(render_bit_def("bit1", true, style)),
        )
        .add(
            Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("class", "background"),
        )
        .add(bits)
}
