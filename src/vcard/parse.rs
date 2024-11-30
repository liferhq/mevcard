use crate::{property::Name, text_field::text_field};

use super::property::Property;

pub fn parse(text: String) -> anyhow::Result<Vec<Property>> {
    if !text.starts_with("BEGIN:VCARD") || !text.ends_with("END:VCARD") {
        return Err(anyhow::anyhow!("Invalid vCard"));
    }

    let lines = text.split("\n").collect::<Vec<_>>();
    let mut i = 0_usize;
    let mut properties: Vec<Property> = vec![];

    macro_rules! next {
        () => {
            i += 1;
            continue;
        };
    }

    while i < lines.len() {
        let line = lines[i];

        if line.starts_with("BEGIN:") {
            next!();
        } else if line.starts_with("END:") {
            break;
        }

        if line.starts_with("VERSION:") {
            properties.push(Property::Version(line[8..].to_string()));
        } else if line.starts_with("FN:") {
            properties.push(Property::FormattedName(line[3..].to_string()));
        } else if line.starts_with("N:") {
            let mut name = Name {
                last_name: None,
                first_name: None,
                additional_name: None,
                name_prefix: None,
                name_suffix: None,
            };

            let chunks = text_field(line[2..].to_string(), ';');
            if chunks.len() != 5 {
                return Err(anyhow::anyhow!(
                    "Invalid vCard (in N property, expected 5 items)"
                ));
            }

            name.last_name = Some(chunks[0].clone());
            name.first_name = Some(chunks[1].clone());
            name.additional_name = Some(chunks[2].clone());
            name.name_prefix = Some(chunks[3].clone());
            name.name_suffix = Some(chunks[4].clone());

            properties.push(Property::Name(name));
        }

        i += 1;
    }

    Ok(properties)
}
