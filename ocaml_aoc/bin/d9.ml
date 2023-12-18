open Core
open Ocaml_aoc

let is_zeros list =
  let rec loop = function
    | [] -> true
    | h :: t -> if h = 0 then loop t else false
  in
  loop list
;;

let get_next_list (list : int list) =
  let rec loop acc = function
    | [] -> acc
    | [ _ ] -> acc
    | h :: m :: t ->
      let tail = m :: t in
      (* don't know why this is not abs if we want the real diff *)
      let diff = m - h in
      loop (acc @ [ diff ]) tail
  in
  loop [] list
;;

let diff_down_to_zero list =
  let rec loop acc list =
    if is_zeros list
    then acc
    else (
      let next_list = get_next_list list in
      loop (next_list :: acc) next_list)
  in
  loop [ list ] list
;;

let get_next_val lists =
  let rec loop acc lists =
    match lists with
    | [] -> acc
    | h :: t ->
      let last =
        match List.rev h |> List.hd with
        | Some x -> x
        | None -> failwith "error there should be a last element"
      in
      let next = acc + last in
      loop next t
  in
  loop 0 lists
;;

let get_prev_val lists =
  let rec loop acc lists =
    match lists with
    | [] -> acc
    | h :: t ->
      let last =
        match List.hd h with
        | Some x -> x
        | None -> failwith "error there should be a last element"
      in
      let next = last - acc in
      loop next t
  in
  loop 0 lists
;;

let get_sum_val list ~get_val =
  let lists = diff_down_to_zero list in
  (* print_endline "lists"; *)
  (* List.iter lists ~f:(fun x -> *)
  (*   List.iter x ~f:(fun x -> printf "%d " x); *)
  (*   printf "\n"); *)
  get_val lists
;;


let parse_to_int_list line =
  let split =
    String.split line ~on:' '
    |> List.map ~f:String.strip
    |> List.filter ~f:(fun x -> not (String.equal x ""))
    |> List.map ~f:Int.of_string
  in
  split
;;

let () =
  let input = read_input_filename "test_input08.txt" in
  printf "input\n";
  List.iter input ~f:print_endline;
  printf "\n";
  let parse_to_int = input |> List.map ~f:parse_to_int_list in
  printf "parse_to_int\n";
  List.iter parse_to_int ~f:(fun x ->
    List.iter x ~f:(fun x -> printf "%d " x);
    printf "\n");
  printf "\n";
  let solve = get_sum_val ~get_val:get_next_val in
  let get_sum_vals = parse_to_int |> List.map ~f:solve in
  printf "get_sum_vals\n";
  List.iter get_sum_vals ~f:(fun x -> printf "%d " x);
  printf "\n";
  let res = get_sum_vals |> List.fold ~init:0 ~f:( + ) in
  printf "res: %d\n" res;
  res |> printf "%d\n"
;;

let () =
  let solve = get_sum_val ~get_val:get_next_val in
  read_input_filename "input08.txt"
  |> List.map ~f:parse_to_int_list
  |> List.map ~f:solve
  |> List.fold ~init:0 ~f:( + )
  |> Int.to_string
  |> print_endline
;;

let () =
  let solve2 = get_sum_val ~get_val:get_prev_val in
  read_input_filename "input08.txt"
  |> List.map ~f:parse_to_int_list
  |> List.map ~f:solve2
  |> List.fold ~init:0 ~f:( + )
  |> Int.to_string
  |> print_endline
;;
