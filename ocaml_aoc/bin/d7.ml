open Core
open Ocaml_aoc

(* advent of code 2023 day 7 *)

type hand =
  { cards : char list
  ; score : int
  ; bid : int
  }

let get_score cards =
  (* get the card counts *)
  let card_counts =
    String.to_list cards
    |> List.sort ~compare:Char.compare
    |> List.group ~break:(fun a b -> not (Char.equal a b))
    |> List.map ~f:(fun x -> List.length x)
  in
  (* get the score *)
  (* get the card counts *)
  match List.sort card_counts ~compare:Int.descending with
  | [ 5 ] -> 7 (* straight flush *)
  | [ 4; 1 ] -> 6 (* four of a kind *)
  | [ 3; 2 ] -> 5 (* full house *)
  | [ 3; 1; 1 ] -> 4 (* three of a kind *)
  | [ 2; 2; 1 ] -> 3 (* two pair *)
  | [ 2; 1; 1; 1 ] -> 2 (* one pair *)
  | [ 1; 1; 1; 1; 1 ] -> 1 (* high card *)
  | _ -> failwith "invalid input"
;;

let card_value ha =
  match ha with
  | 'A' -> 14
  | 'K' -> 13
  | 'Q' -> 12
  | 'J' -> 11
  | 'T' -> 10
  | _ as card -> int_of_char card - int_of_char '0'
;;

let card_compare a b =
  let rec aux a b =
    match a, b with
    | [], [] -> 0
    | ha :: ta, hb :: tb ->
      if Char.equal ha hb
      then aux ta tb
      else (
        let a_val = card_value ha in
        let b_val = card_value hb in
        a_val - b_val)
    | _ -> failwith "invalid input"
  in
  aux a b
;;

let compare a b =
  let res =
    if a.score <> b.score
    then a.score - b.score
    else card_compare a.cards b.cards
  in
  res
;;

let parse_card line =
  let parts = String.split line ~on:' ' |> List.map ~f:String.strip in
  match parts with
  | [ cards; bid ] ->
    { cards = String.to_list cards
    ; score = get_score cards
    ; bid = Int.of_string bid
    }
  | _ -> failwith "invalid input"
;;

let parse_cards_from_input input =
  let rec aux acc = function
    | [] -> acc
    | h :: t ->
      let card = parse_card h in
      aux (card :: acc) t
  in
  aux [] input
;;

let process input name =
  input
  |> parse_cards_from_input
  |> List.sort ~compare
  |> List.rev
  |> List.foldi ~f:(fun i acc hand -> acc + (hand.bid * (i + 1))) ~init:0
  |> printf "%s %d\n" name
;;

let () =
  let input = read_input_filename "test_input07_all_cards.txt" in
  let sorted =
    input |> parse_cards_from_input |> List.sort ~compare |> List.rev
  in
  List.iter sorted ~f:(fun x ->
    printf "%s %d %d\n" (String.of_char_list x.cards) x.score x.bid);
  sorted
  |> List.foldi ~f:(fun i acc hand -> acc + (hand.bid * (i + 1))) ~init:0
  |> printf "Day 7 %d\n";
  process (read_input_filename "input07.txt") "Day 7 real"
;;
