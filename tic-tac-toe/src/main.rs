fn main() {
    println!("This is the start of the code :::::");
    let mut board = [' '; 9];
    let player = [ 'X', 'O'];

    let mut turn = 0;
    
    loop{
        let index = get_index_from_input();
        if let Err(e) = index{
            println!("{e}");
            continue;
        }
        let index = index.unwrap();

        if let None = index {
            break;
        }

        let index = index.unwrap();

        if board[index] != ' ' {
            println!("you cant rewrite the already marked position");
            continue;
        }

        board[index] = player[turn];
        print_board(board);

        turn = (turn + 1 ) % 2 ;
    }


}

fn print_board(board: [char;9]){
    println!("
    +----+----+----+
    | {} | {} |{} |
    +----+----+----+
    | {} | {} | {} |
    +----+----+----+
    | {} | {} | {} |
    +----+----+----+",board[0],board[1],board[2],board[3],board[4],board[5],board[6],board[7],board[8])

}

fn get_index_from_input() -> Result<Option<usize>, String>{
    let mut input =String::new();
    let _ = std::io::stdin().read_line(&mut input)
    .map_err(|e| format!("Error reading input :::{}",e))?;
    let input = input.trim();

    if input == "quit"{
        return Ok(None)
    }

    let index = input.parse::<usize>()
    .map_err( |_| format!("{} it is not a valid number ", input) )?;

    if index < 1 || index > 9 {
        return Err(format!("the index should be in between 1 and 9"))
    }
    Ok(Some(index -1 ))
}
