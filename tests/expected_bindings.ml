type nonrec single_tuple = { inner: string } [@@boxed]
external new_t : unit -> single_tuple = "new"
external print_t : single_tuple -> unit = "print"
type nonrec key_value = { map: ((string * string) list) } [@@boxed]
type nonrec list_t = { items: ((string) list) } [@@boxed]

module Car = struct 
  type nonrec t
end

external fn_one_parameter : Car.t -> Car.t = "fn_one_parameter"
external fn_two_parameters : Car.t -> int -> Car.t = "fn_two_parameters"
external fn_three_parameters : Car.t -> int -> int -> Car.t = "fn_three_parameters"
external fn_four_parameters : Car.t -> int -> int -> int -> Car.t = "fn_four_parameters"
external fn_five_parameters : Car.t -> int -> int -> int -> int -> Car.t = "fn_five_parameters"
external fn_six_parameters : Car.t -> int -> int -> int -> int -> int -> Car.t = "fn_six_parameters_bytecode" "fn_six_parameters"
external test_add_i32 : int32 -> int32 -> int32 = "test_add_i32"
external test_add_usize : int -> int -> int = "test_add_usize"
external test_bytes_get : bytes -> int -> int = "test_bytes_get"
external test_get_ascii_code : int -> int32 = "test_get_ascii_code"

module Toyota = struct 
  type nonrec t = Car.t
  external create_toyota : unit -> t = "create_toyota"
end

