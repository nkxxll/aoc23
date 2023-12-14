open Core
open Ocaml_aoc

let parse_line line =
  let split = String.split line ~on:':' in
  match split with
  | [ a; b ] ->
    String.strip b
    |> String.split ~on:' '
    |> List.filter ~f:(fun x -> if String.equal x "" then false else true)
    |> List.map ~f:String.strip
    |> List.map ~f:int_of_string
  | _ -> failwith "Invalid input"
;;

let parse_line_long line =
  let split = String.split line ~on:':' in
  match split with
  | [ a; b ] ->
    let long_int =
      String.strip b |> String.split ~on:' ' |> String.concat |> int_of_string
    in
    [ long_int ]
  | _ -> failwith "Invalid input"
;;

let parse_input_with input ~f =
  match input with
  | [ a; b ] -> f a, f b
  | _ -> failwith "Invalid input"
;;

let is_greater time full_time distance =
  let calc_distance = time * (full_time - time) in
  if calc_distance > distance then true else false
;;

let get_score time distance =
  let full_time = time in
  let rec aux acc time =
    match time with
    | 0 -> acc
    | _ ->
      if is_greater time full_time distance
      then aux (1 + acc) (time - 1)
      else aux acc (time - 1)
  in
  aux 0 time
;;

let find_greater_distances time_distance_list =
  (* we get a list of [ times; distances ] *)
  let times, distances = time_distance_list in
  let rec find_greater_distances acc times distances =
    match times, distances with
    | [], [] -> acc
    | t :: ts, d :: ds ->
      let greater = get_score t d in
      find_greater_distances (greater :: acc) ts ds
    | _ -> failwith "Invalid input"
  in
  find_greater_distances [] times distances
;;

let () =
  read_input 6
  |> parse_input_with ~f:parse_line
  |> find_greater_distances
  |> mul_ints
  |> string_of_int
  |> print_endline
;;

let () =
  read_input 6
  |> parse_input_with ~f:parse_line_long
  |> find_greater_distances
  |> mul_ints
  |> string_of_int
  |> print_endline
;;

