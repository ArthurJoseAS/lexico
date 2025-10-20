use std::io;

struct Analisador<'a> {
    ///é a posição inicial em caracteres do proximo elemento lexico a ser lido
    pos: usize,
    ///é a fatia contendo todos os elementos não lidos ainda
    prox: &'a str,
}

impl<'a> Analisador<'a>{
    fn novo(entrada: &'a str, pos: usize) -> Self{
        Self{pos: pos, prox: &entrada}
    }
    /**
     * Altera o pos para ser o inicio do elemento léxico que vem depois do elemento lido
     * Altera prox para ser uma subfatia, eliminando o que foi retornado como elemento lexico
     * Retorna o elemento léxico lido
     * Retorna a posição de inicio do elemento léxico lido
     */
    fn proximo(&mut self) -> Result<(usize, &str), Option<usize>>{ 
        
        let mut slice_begin_bytes: usize = 0;
        let mut slice_begin_chars : usize = self.pos;

        let mut char_indx: usize = self.pos;
        let mut byte_indx_outside_for_scope: usize = 0;

        let mut op_was_consumed= false;
        let mut empty_was_consumed = false;
        let mut digit_was_consumed = false;
        for (byte_indx, character) in self.prox.char_indices(){
            byte_indx_outside_for_scope = byte_indx;
            if empty_was_consumed{
                slice_begin_bytes = byte_indx;
                slice_begin_chars = char_indx;
                empty_was_consumed = false;
            }
            if digit_was_consumed{
                //consumir mais um digito até acabarem os digitos
                if !character.is_numeric(){
                    if slice_begin_bytes == 0{
                        let return_pos = slice_begin_chars;
                        let return_slice =  &self.prox[..byte_indx];
                        self.prox = &self.prox[byte_indx..];
                        self.pos = char_indx;
                        return Ok((return_pos, return_slice));
                        // return (slice_begin_chars, char_indx, &self.prox[..byte_indx], &self.prox[byte_indx..]);
                        
                    }
                    else{
                        let return_pos = slice_begin_chars;
                        let return_slice =  &self.prox[slice_begin_bytes..byte_indx];
                        self.prox = &self.prox[byte_indx..];
                        self.pos = char_indx;
                        return Ok((return_pos, return_slice));
                        // return (slice_begin_chars, char_indx, &self.prox[slice_begin_bytes..byte_indx], &self.prox[byte_indx..]);
                    }
                }
                else{
                    char_indx+=1;
                    continue;
                }
            }
            else if op_was_consumed{
                    if slice_begin_bytes == 0{
                        let return_pos = slice_begin_chars;
                        let return_slice =  &self.prox[..byte_indx];
                        self.prox = &self.prox[byte_indx..];
                        self.pos = char_indx;
                        return Ok((return_pos, return_slice));
                        // return (slice_begin_chars, char_indx, &self.prox[..byte_indx], &self.prox[byte_indx..]);
                        
                    }
                    else{
                        let return_pos = slice_begin_chars;
                        let return_slice =  &self.prox[slice_begin_bytes..byte_indx];
                        self.prox = &self.prox[byte_indx..];
                        self.pos = char_indx;
                        return Ok((return_pos, return_slice));
                        // return (slice_begin_chars, char_indx, &self.prox[slice_begin_bytes..byte_indx], &self.prox[byte_indx..]);
                    }

            }
            
            if is_operation(character){
                op_was_consumed = true;
            }
            else if character.is_numeric(){
                digit_was_consumed = true;
            }
            else if character == '🦀' || character == ' '{
                empty_was_consumed = true;
            }
            else{
                return Err(Some(char_indx+1));
            }
            char_indx+=1;
        }
        if digit_was_consumed {
            if slice_begin_bytes == 0{
                let return_pos = slice_begin_chars;
                let return_slice =  &self.prox[..=byte_indx_outside_for_scope];
                self.prox = &self.prox[..0];
                self.pos = char_indx;
                return Ok((return_pos, return_slice));
                // return Ok((slice_begin_chars, char_indx, &self.prox[..=byte_indx_outside_for_scope], &self.prox[..0]));
            }
            else{
                let return_pos = slice_begin_chars;
                let return_slice =  &self.prox[slice_begin_bytes..=byte_indx_outside_for_scope];
                self.prox = &self.prox[..0];
                self.pos = char_indx;
                return Ok((return_pos, return_slice));
                // return Ok((slice_begin_chars, char_indx, &self.prox[slice_begin_bytes..=byte_indx_outside_for_scope], &self.prox[..0]));
            }
        }
        else if op_was_consumed {
            if slice_begin_bytes == 0{
                let return_pos = slice_begin_chars;
                let return_slice =  &self.prox[..=byte_indx_outside_for_scope];
                self.prox = &self.prox[..0];
                self.pos = char_indx;
                return Ok((return_pos, return_slice));
                // return Ok((slice_begin_chars, char_indx, &self.prox[..=byte_indx_outside_for_scope], &self.prox[..0]));
            }
            else{
                let return_pos = slice_begin_chars;
                let return_slice =  &self.prox[slice_begin_bytes..=byte_indx_outside_for_scope];
                self.prox = &self.prox[..0];
                self.pos = char_indx;
                return Ok((return_pos, return_slice));
                // return Ok((slice_begin_chars, char_indx, &self.prox[slice_begin_bytes..=byte_indx_outside_for_scope], &self.prox[..0]));
            }
            // return Ok((slice_begin_chars, char_indx, &entrada[..=byte_indx_outside_for_scope], &entrada[..0]))
        }
        return Err(Some(char_indx));
    }
}

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
        buf = buf.trim_matches(&['🦀', ' ', '\n', '\t']).to_string();
        let mut anl: Analisador = Analisador::novo(&buf, 0);
        while !anl.prox.is_empty(){
            // print!(" |{}| ", str_slice);
            match anl.proximo(){
                Ok(a) => {
                    print!("(\"{}\", {}) ", a.1, a.0+1);
                }
                Err(e) =>{
                    println!("Erro na posição {}", e.unwrap());
                    break;
                }
            }
        }
        buf.clear();
        println!("");
    }
}
