use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlWriteEvent, XmlEventWriter},
};

pub fn serialize_udim<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: (f32, i32),
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("UDim").attr("name", name))?;

    writer.write_tag_characters("S", value.0)?;
    writer.write_tag_characters("O", value.1)?;

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn serialize_udim2<W: Write>(
    writer: &mut XmlEventWriter<W>,
    name: &str,
    value: (f32, i32, f32, i32),
) -> Result<(), EncodeError> {
    writer.write(XmlWriteEvent::start_element("UDim2").attr("name", name))?;

    writer.write_tag_characters("XS", value.0)?;
    writer.write_tag_characters("XO", value.1)?;
    writer.write_tag_characters("YS", value.2)?;
    writer.write_tag_characters("YO", value.3)?;

    writer.write(XmlWriteEvent::end_element())?;

    Ok(())
}

pub fn deserialize_udim<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("UDim")?;

    let scale: f32 = reader.read_tag_contents("S")?.parse()?;
    let offset: i32 = reader.read_tag_contents("O")?.parse()?;

    reader.expect_end_with_name("UDim")?;

    Ok(RbxValue::UDim {
        value: (scale, offset),
    })
}

pub fn deserialize_udim2<R: Read>(reader: &mut EventIterator<R>) -> Result<RbxValue, DecodeError> {
    reader.expect_start_with_name("UDim2")?;

    let x_scale: f32 = reader.read_tag_contents("XS")?.parse()?;
    let x_offset: i32 = reader.read_tag_contents("XO")?.parse()?;
    let y_scale: f32 = reader.read_tag_contents("YS")?.parse()?;
    let y_offset: i32 = reader.read_tag_contents("YO")?.parse()?;

    reader.expect_end_with_name("UDim2")?;

    Ok(RbxValue::UDim2 {
        value: (x_scale, x_offset, y_scale, y_offset),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_trip_udim() {
        let _ = env_logger::try_init();

        let test_input = (0.5, 1);
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_udim(&mut writer, "foo", test_input).unwrap();

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_udim(&mut reader).unwrap();

        assert_eq!(value, RbxValue::UDim {
            value: test_input,
        });
    }

    #[test]
    fn round_trip_udim2() {
        let _ = env_logger::try_init();

        let test_input = (0.5, 1, 1.5, 2);
        let mut buffer = Vec::new();

        let mut writer = XmlEventWriter::from_output(&mut buffer);
        serialize_udim2(&mut writer, "foo", test_input).unwrap();

        let mut reader = EventIterator::from_source(buffer.as_slice());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_udim2(&mut reader).unwrap();

        assert_eq!(value, RbxValue::UDim2 {
            value: test_input,
        });
    }

    #[test]
    fn de_udim() {
        let _ = env_logger::try_init();

        let buffer = r#"
            <UDim>
                <S>0.5</S>
                <O>1</O>
            </UDim>
        "#;

        let mut reader = EventIterator::from_source(buffer.as_bytes());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_udim(&mut reader).unwrap();

        assert_eq!(value, RbxValue::UDim {
            value: (0.5, 1),
        });
    }

    #[test]
    fn de_udim2() {
        let _ = env_logger::try_init();

        let buffer = r#"
            <UDim2>
                <XS>0.5</XS>
                <XO>1</XO>
                <YS>1.5</YS>
                <YO>2</YO>
            </UDim2>
        "#;

        let mut reader = EventIterator::from_source(buffer.as_bytes());
        reader.next().unwrap().unwrap(); // Eat StartDocument event
        let value = deserialize_udim2(&mut reader).unwrap();

        assert_eq!(value, RbxValue::UDim2 {
            value: (0.5, 1, 1.5, 2),
        });
    }
}