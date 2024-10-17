use itertools::Itertools;
use svg::{
    node::element::{Definitions, Group, Rectangle, Text, Use},
    Document,
};

pub struct RenderStyle {
    pub group_size: usize,
    pub line_size: usize,
    pub group_padding: usize,
    pub line_padding: usize,
    pub box_size: usize,
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

pub fn render_bits_doc<I: Iterator<Item = bool>>(iter: I, style: &RenderStyle) -> Document {
    let (dimx, dimy, bits) = render_bits(iter, style);
    Document::new()
        .set("width", dimx)
        .set("height", dimy)
        .add(
            Definitions::new()
                .add(
                    Group::new()
                        .set("id", "bit0")
                        .add(
                            Rectangle::new()
                                .set("height", style.box_size)
                                .set("width", style.box_size)
                                .set("x", 0)
                                .set("y", 0)
                                .set("fill", "white"),
                        )
                        .add(
                            Text::new("0")
                                .set("x", style.box_size / 2)
                                .set("y", style.box_size / 2)
                                .set("dy", "0.35em")
                                .set("fill", "black")
                                .set("font-size", style.box_size)
                                .set("font-family", "monospace"),
                        ),
                )
                .add(
                    Group::new()
                        .set("id", "bit1")
                        .add(
                            Rectangle::new()
                                .set("height", style.box_size)
                                .set("width", style.box_size)
                                .set("fill", "white"),
                        )
                        .add(
                            Text::new("1")
                                .set("x", style.box_size / 2)
                                .set("y", style.box_size / 2)
                                .set("dy", "0.35em")
                                .set("fill", "black")
                                .set("font-size", style.box_size)
                                .set("font-family", "monospace"),
                        ),
                ),
        )
        .add(bits)
}
