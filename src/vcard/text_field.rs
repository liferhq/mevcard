/// Returns a vector of chunks of text separated by `splitter` while taking into account escape characters like `\n`, `\,` and `\;`.
pub fn text_field(text: String, splitter: char) -> Vec<String> {
    let chars = text.chars().collect::<Vec<_>>();

    let mut i = 0_usize;
    let mut chunks = vec!["".to_string()];
    let mut collector = &mut chunks[0];

    while i < text.len() {
        if chars[i] == '\\' {
            if let Some(&n) = chars.get(i + 1) {
                match n {
                    'n' => {
                        collector.push('\n');
                        i += 2;
                        continue;
                    }
                    ',' => {
                        collector.push(',');
                        i += 2;
                        continue;
                    }
                    ';' => {
                        collector.push(';');
                        i += 2;
                        continue;
                    }
                    '\\' => {
                        collector.push('\\');
                        i += 2;
                        continue;
                    }
                    _ => (),
                }
            }
        } else if chars[i] == splitter {
            let thislen = chunks.len();
            chunks.push("".to_string());
            collector = &mut chunks[thislen];
        } else {
            collector.push(chars[i]);
        }

        i += 1;
    }

    chunks
}
