#[doc="Text layout."]

import gfx::geom::au;
import /*layout::*/base::*; // FIXME: Can't get around import *; resolve bug.
import servo_text::text_run::text_run;

class text_box {
    let text: str;
    let mut run: option<text_run>;

    new(text: str) {
        self.text = text;
        self.run = none;
    }
}

#[doc="The main reflow routine for text layout."]
impl text_layout_methods for @box {
    fn reflow_text(_available_width: au, subbox: @text_box) {
        alt self.kind {
            bk_text(*) { /* ok */ }
            _ { fail "expected text box in reflow_text!" }
        };

        let run = text_run(subbox.text);
        subbox.run = some(run);
        run.shape();

        self.bounds.size = {
            mut width:
                alt vec::last_opt(run.glyphs.get()) {
                    some(glyph) {
                        au(*glyph.pos.offset.x + *glyph.pos.advance.x)
                    }
                    none { au(0) }
                },
            mut height: au(60 * 14)
        };
    }
}
