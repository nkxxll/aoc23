open Core
open Ocaml_aoc

let print_lines lines =
  List.iter lines ~f:(fun line -> print_endline line)
;;

let line_to_value line =
  let rec aux (acc : char list) (char_list : char list) =
    match char_list with
    (* return the first and the last element of the acc list concatenated as int *)
    | [] ->
      (match acc with
       | [] -> 0
       | _ as acc ->
         let string =
           (List.hd_exn acc |> Char.to_string)
           ^ (List.last_exn acc |> Char.to_string)
         in
         string |> Int.of_string)
    (* if the head is a digit, add it to the accumulator, otherwise return the accumulator *)
    | [ h ] ->
      aux
        (match h with
         | '0' .. '9' as c -> acc @ [ c ]
         | _ -> acc)
        []
    (* if the head is a digit, add it to the accumulator, otherwise return the accumulator *)
    | h :: t ->
      aux
        (match h with
         | '0' .. '9' as c -> acc @ [ c ]
         | _ -> acc)
        t
  in
  aux [] (String.to_list line)
;;

let get_calibration_value lines =
  let rec aux acc lines =
    match lines with
    | [] -> acc
    | h :: t -> aux (acc + line_to_value h) t
  in
  aux 0 lines
;;

(* let () = read_input 1 |> List.iter ~f:Stdio.print_endline *)
let () =
  read_input 1 |> get_calibration_value |> string_of_int |> Stdio.print_endline
;;
(* print lines *)
