# Fulcrum 0.3.0
A fast and straight forward scripting language with a tiny binary.

## Documentation

## Features:

 - Higher order functions.

 - Lists.

 - An extremely minimal and basic syntax.

 - An interpreter that is under 1 megabyte in size.

## Language Rules:

 - Assignments, function calls, and return statements must be ended with a `;`

 - Operators are not yet a part of the language but operations can be done via functions.

 - Functions defined inside of other functions only are callable inside of the parent function and inherit all of the parent function's variables and function definitions.

 - Higher order functions must include a function name, this requirement will be taken out once a new syntax for function literals is added.

 - When exiting any scope, all variables defined within that scope are deleted.

 - Nothing is passed by reference, all variables are modified via the `=` asignment operator.

## Language Keywords and syntax:

 - `fun` : Prefixes a function definition.

 - `return` : Returns a value from a user defined function.

 - `if` : Prefixes the condition for an if block.

 - `elif` : Prefixes the condition for an elif block.

 - `else` : Prefixes an else block.

 - `[]` : Wraps a list literal or is used to hold the offset for an index/offset operator.

 - `=` : Used to declare or assign to existing variables.

 - `"` : Wraps a string literal.

 - `;` : Denotes the end of a line.

 - `()` : Suffixes a function name when calling or declaring that function. Wraps function arguments when calling a function and wraps function argument names when declaring a function.

 - `true`|`yes`|`t` : Aliases for the boolean literal `true`.

 - `false`|`no`|`f` : Aliases for the boolean literal `false`.

## Standard Library Functions:

Functions in fulcrum return the result of their operation.

 - `print(val)` : Prints a value to the standard output.

 - `input()` : Gets user input.

 - `read(filepath)` : Returns the contents of the file provided.

 - `write(filepath, new_content, writemode)` : Writes the `new_content` to the file provided based on the writemode which can be `"a"` for append or `"t"` for truncate.

 - `add(val1, val2)` : Adds 2 values. also concatenates strings.

 - `sub(val1, val2)` : Subtracts 2 values. also removes substrings from strings.

 - `mul(val1, val2)` : Multiplies 2 values.

 - `div(val1, val2)` : divides 2 values.

 - `E(val1, val2)` : Tests if two values are equal and returns boolean.

 - `NE(val1, val2)` : Tests if two values are not equal and returns boolean.

 - `G(val1, val2)` : Tests if `val1` is greater than `val2`.

 - `L(val1, val2)` : Tests if `val1` is less than `val2`.

 - `GE(val1, val2)` : Tests if `val1` is greater than or equal to `val2`.

 - `LE(val1, val2)` : Tests if `val1` is less than or equal to `val2`.

 - `and(condition1, condition2)` : Tests if both conditions are true and returns boolean.

 - `or(condition1, condition2)` : Tests if one of the conditions are true and returns boolean.

 - `INT(val)` : Casts the type to an Int.

 - `FLOAT(val)` : Casts the type to a Float.

 - `BOOL(val)` : Casts the type to a Boolean.

 - `STRING(val)` : Casts the type to a String.

## Example Program:

Tic Tac Toe:
```
# tictactoe.ful

fun tictactoe() {
	# Board will act as a global that will hold the state of the tic tac toe board.
	board = [
		# 1   2   3
		[" "," "," "], #1
		[" "," "," "], #2
		[" "," "," "]  #3
	];

	fun update_board(current_player) {
		# This function starts a turn for a player.

		fun print_board() {
			# this function renders the current board.
			# board is inherited from top most function and acts as a global.
			print(add(add(add(add(board[0][0], "|"),board[0][1]),"|"),board[0][2]));
			print("\n-----\n");
			print(add(add(add(add(board[1][0], "|"),board[1][1]),"|"),board[1][2]));
			print("\n-----\n");
			print(add(add(add(add(board[2][0], "|"),board[2][1]),"|"),board[2][2]));
			print("\n");
		}

		# render the board at the start of the turn
		print_board();

		# Get horizontal input.
		print("horizontal:");
		x = sub(INT(input()),1);
		
		# Get vertical input.
		print("vertical:");
		y = sub(INT(input()), 1);
		
		# Set the current player at that position.
		board[x][y] = current_player;
		
		# Check who the current player is to recursively call update board as the other player.
		if E(current_player, "X") {
			update_board("O");
		}
		else {
			update_board("X");
		}
	}
	update_board("X");
}

# Run the game
tictactoe();
```