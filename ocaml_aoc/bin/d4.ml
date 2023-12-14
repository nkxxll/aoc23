open Core
open Ocaml_aoc

let split_striped line =
  line
  |> String.strip
  |> String.split ~on:' '
  |> List.filter ~f:(fun x -> not (String.is_empty x))
;;

let get_winning_numbers line =
  match String.split line ~on:'|' with
  | [ x; y ] -> [ split_striped x, split_striped y ]
  | _ -> failwith "invalid input"
;;

let calc_points winning numbers =
  numbers
  |> List.fold ~init:0 ~f:(fun acc x ->
    match List.find winning ~f:(fun y -> String.equal x y) with
    | Some _ -> if acc = 0 then 1 else acc * 2
    | None -> acc)
;;

let preporcess_line line =
  match String.strip line |> String.split ~on:':' with
  | [ x; y ] -> y |> String.strip
  | _ -> failwith "invalid input"
;;

let get_score (line : string) =
  let line = preporcess_line line in
  let [ (winning, numbers) ] = get_winning_numbers line in
  calc_points winning numbers
;;

let count_matching winning numbers =
  numbers
  |> List.fold ~init:0 ~f:(fun acc x ->
    match List.find winning ~f:(fun y -> String.equal x y) with
    | Some _ -> acc + 1
    | None -> acc)
;;

let get_card_count line =
  let line = preporcess_line line in
  let [ (winning, numbers) ] = get_winning_numbers line in
  count_matching winning numbers
;;

let sum x =
  let rec aux acc = function
    | [] -> acc
    | x :: xs -> aux (acc + x) xs
  in
  aux 0 x
;;

let range start stop f = List.range start stop |> List.iter ~f

(* print the input *)
let () =
  let scores = read_input 4 |> List.map ~f:get_score in
  scores |> sum |> printf "solution uno: %d\n"
;;

let () =
  let scores = read_input 4 |> List.map ~f:get_card_count in
  let multiplers = Array.create ~len:(List.length scores) 1 in
  List.iteri scores ~f:(fun idx score ->
    range 1 (score + 1) (fun offset ->
      let card = offset + idx in
      multiplers.(card) <- multiplers.(card) + multiplers.(idx)));
  printf "Part 2: %d.\n" (Array.reduce multiplers ~f:( + ) |> Option.value_exn)
;;
