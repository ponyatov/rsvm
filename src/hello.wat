;; hello world

(module
    ;; Import the JavaScript console.log function
    (import "console" "log" (func $log (param i32)))

    ;; shared memory region (size in 64K pages)
    (memory (export "memory") 1)

    ;; data section
    (data (i32.const 0) "Hello, World!\00")

    ;; export func to be called from Js
    (func (export "hello")
        ;; call JS function
        (call $log (i32.const 0))
    )

;;    (func (export "add") (param $n1 i32) (param $n2 i32) (result i32)
;;        (get_local $n1)
;;        (get_local $n2)
;;        (i32.add)
;;    )
)
