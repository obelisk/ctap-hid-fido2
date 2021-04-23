use crate::bio_enrollment_params::{BioEnrollmentData, TemplateInfo};
#[allow(unused_imports)]
use crate::util;
use serde_cbor::Value;

pub(crate) fn parse_cbor(bytes: &[u8]) -> Result<BioEnrollmentData, String> {
    let mut data = BioEnrollmentData::default();
    let maps = util::cbor_bytes_to_map(bytes)?;
    for (key, val) in &maps {
        if let Value::Integer(member) = key {
            match member {
                0x01 => data.modality = util::cbor_value_to_num(val)?,
                0x02 => data.fingerprint_kind = util::cbor_value_to_num(val)?,
                0x03 => {
                    data.max_capture_samples_required_for_enroll = util::cbor_value_to_num(val)?
                }
                0x07 => {
                    if let Value::Array(xs) = val {
                        for x in xs {
                            let mut template = TemplateInfo::default();
                            template.template_id = util::cbor_get_bytes_from_map(x, "1")?;
                            template.template_friendly_name =
                                util::cbor_get_string_from_map(x, "2")?;
                            data.template_infos.push(template);
                        }
                    }
                }
                0x08 => data.max_template_friendly_name = util::cbor_value_to_num(val)?,
                _ => println!("parse_cbor_member - unknown info {:?}", member),
            }
        }
    }
    Ok(data)
}
