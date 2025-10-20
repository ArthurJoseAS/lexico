use std::io;


fn is_operation(character: char) -> bool{
    match character{
        '+' =>{
            return true;
        }
        '-' => {
            return true;
        }
        '*' => {
            return true;
        }
        '/' => {
            return true;
        }
        '🐧' => {
            return true;
        }
        _ =>{
            return false;
        }

    }
}


/**Retorna

Ok((posição do primeiro caracter do elemento lexico lido em caracteres, 
posição em caracteres da proxima fatia da string a ser lida
fatia da string do elemento lexico lido,
fatia da string para analise))

Err(posição do caracter com erro)
*/
fn proximo(entrada: &str, begin_bytes: usize, begin_chars: usize) -> Result <(usize, usize ,&str, &str), Option<usize>>{
    fn ok_return_expression(entrada: &str, slice_begin_chars: usize, char_indx: usize, slice_begin_bytes: usize, byte_indx: usize, begin_bytes: usize) -> (usize, usize ,&str, &str){
        if slice_begin_bytes == begin_bytes{
            return (slice_begin_chars, char_indx, &entrada[..byte_indx], &entrada[byte_indx..]);
        }
        else{
            return (slice_begin_chars, char_indx, &entrada[slice_begin_bytes..byte_indx], &entrada[byte_indx..]);
        }
    }
    
    let mut slice_begin_bytes: usize = begin_bytes;
    let mut slice_begin_chars : usize = begin_chars;

    let mut char_indx: usize = begin_chars;
    let mut byte_indx_outside_for_scope: usize = begin_bytes;

    let mut op_was_consumed= false;
    let mut empty_was_consumed = false;
    let mut digit_was_consumed = false;
    for (byte_indx, character) in entrada.char_indices(){
        byte_indx_outside_for_scope = byte_indx;
        if empty_was_consumed{
            slice_begin_bytes = byte_indx;
            slice_begin_chars = char_indx;
            empty_was_consumed = false;
        }
        if digit_was_consumed{
            //consumir mais um digito até acabarem os digitos
            if !character.is_numeric(){
                return Ok(ok_return_expression(entrada, slice_begin_chars, char_indx, slice_begin_bytes, byte_indx, begin_bytes));
            }
            else{
                char_indx+=1;
                continue;
            }
        }
        else if op_was_consumed{
            return Ok(ok_return_expression(entrada, slice_begin_chars, char_indx, slice_begin_bytes, byte_indx, begin_bytes));
        }
        
        if is_operation(character){
            op_was_consumed = true;
        }
        else if character.is_numeric(){
            digit_was_consumed = true;
        }
        else if character == '🦀'{
            empty_was_consumed = true;
        }
        else if character == ' '{
            empty_was_consumed = true;
        }
        else{
            return Err(Some(char_indx+1));
        }
        char_indx+=1;
    }
    if digit_was_consumed {
        if slice_begin_bytes == begin_bytes{
            return Ok((slice_begin_chars, char_indx, &entrada[..=byte_indx_outside_for_scope], &entrada[..0]));
        }
        else{
            return Ok((slice_begin_chars, char_indx, &entrada[slice_begin_bytes..=byte_indx_outside_for_scope], &entrada[..0]));
        }
    }
    else if op_was_consumed {
        if slice_begin_bytes == begin_bytes{
            return Ok((slice_begin_chars, char_indx, &entrada[..=byte_indx_outside_for_scope], &entrada[..0]));
        }
        else{
            return Ok((slice_begin_chars, char_indx, &entrada[slice_begin_bytes..=byte_indx_outside_for_scope], &entrada[..0]));
        }
        // return Ok((slice_begin_chars, char_indx, &entrada[..=byte_indx_outside_for_scope], &entrada[..0]))
    }
    return Err(Some(char_indx));
}
fn main() -> io::Result<()> {
    let mut buf: String = String::new();
    loop{
        match io::stdin().read_line(&mut buf){
            Ok(n) => {
                if n == 0 {
                    return Ok(());
                }
            }
            Err(e) => {
                println!("Erro no read_line");
                return Err(e);
            }
        }
        buf = buf.trim().to_string();
        buf = buf.trim_matches(&['🦀', ' ', '\n', 't']).to_string();
        let mut str_slice = &buf[0..];

        let mut begin_bytes: usize = 0;
        let mut begin_chars: usize = 0;
        // print!(" {} ", str_slice);
        while !str_slice.is_empty(){
            // print!(" |{}| ", str_slice);
            match proximo(str_slice, begin_bytes, begin_chars){
                Ok(a) => {
                    begin_bytes = str_slice.len() - a.3.len();
                    str_slice = a.3;
                    print!("(\"{}\", {}) ", a.2, a.0+1);
                    begin_chars = a.1;
                }
                Err(e) =>{
                    println!("Erro na posição {}", e.unwrap());
                    return Ok(());
                }
            }
        }
        buf.clear();
        println!("");
    }
}
