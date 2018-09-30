type keypress = float * string

val make_stream : unit -> keypress Lwt_stream.t Lwt.t