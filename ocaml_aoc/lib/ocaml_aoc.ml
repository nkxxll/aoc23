open Core

let sum_ints = List.fold ~init:0 ~f:( + )
let mul_ints = List.fold ~init:1 ~f:( * )

let read_input day =
  let filename = Printf.sprintf "inputs/input%02d.txt" day in
  let lines = Stdio.In_channel.read_lines filename in
  lines
  |> List.filter ~f:(fun line ->
    match line with
    | "" -> false
    | _ -> true)
;;
