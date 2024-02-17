use encoding_rs::CoderResult;

pub fn decode_bytes(bytes: &[u8]) -> String {
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, true);
    let encoding = detector.guess(None, true);
    let mut decoder = encoding.new_decoder();
    let mut buffer_bytes = [0u8; 8];
    let buffer: &mut str = std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap();
    let mut _total_had_errors = false;
    let mut bytes_in_buffer = 0usize;
    let mut output = String::new();

    let mut total_read_from_current_input = 0usize;

    loop {
        let (result, read, written, had_errors) = decoder.decode_to_str(
            &bytes[total_read_from_current_input..],
            &mut buffer[bytes_in_buffer..],
            false,
        );
        total_read_from_current_input += read;
        bytes_in_buffer += written;
        _total_had_errors |= had_errors;
        match result {
            CoderResult::InputEmpty => {
                // We have consumed the current input buffer. Break out of
                // the inner loop to get the next input buffer from the
                // outer loop.
                break;
            }
            CoderResult::OutputFull => {
                // Write the current buffer out and consider the buffer
                // empty.
                output.push_str(&buffer[..bytes_in_buffer]);
                bytes_in_buffer = 0usize;
                continue;
            }
        }
    }

    output
}
