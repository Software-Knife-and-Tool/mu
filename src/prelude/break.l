;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; debugging
;;;
#|
(mu:intern prelude "%debug-map-symbol"
   (:lambda (value)
      (:if (mu:boundp 'prelude:%debug-symbols%)
           ()
           (mu:intern prelude "%debug-symbols%" (mu:ns-syms :list prelude)))
      (mu:fix
       (:lambda (list)
          (:if (prelude:consp list)
               ((:lambda (symbol)
                   (:if (mu:eq (mu:symbol-value symbol) value)
                        symbol
                        (mu:cdr list)))
                (mu:car list))
               list))
       prelude:%debug-symbols%)))

(mu:intern prelude "%format-mapped-symbol"
   (:lambda (value)
     ((:lambda (symbol)
        (:if (prelude:null symbol)
             value
             symbol))
       (prelude:%debug-map-symbol value))))
|#

;;;
;;; break loop
;;;
(mu:intern prelude "break"
   (:lambda (exception)
     (core:format :t ";;; entering break loop with exception~%" ())
     (core:%exceptionf :t ";;; ~A on ~A by ~S, ~A~%" () exception)
     (core:format :t ";;; :h for commands~%" ())
     (mu:fix
      (:lambda (loop)
        ((:lambda (form)
           (:if (mu:eq form core:%eof%)
                loop
                (:if (mu:eq form :h)
                     ((:lambda ()
                        (core:format :t "break help:~%" ())
                        (core:format :t ":d - describe exception value~%" ())
                        (core:format :t ":e - print the full exception~%" ())
                        (core:format :t ":r - return from break~%" ())
                        (core:format :t ":x - exit process~%" ())
                        (core:null loop)))
                     (:if (mu:eq form :d)
                          ((:lambda (value)
                             ;;; (prelude:describe (mu:cdr value) :t)
                             (core:null loop))
                           (core:%exception-prop :value except))
                          (:if (mu:eq form :x)
                               (mu:exit 0)
                               (:if (mu:eq form :e)
                                    ((:lambda ()
                                       (core:%exceptionf mu:*standard-output* "exception: ~A on ~A by ~S, ~A~%" :t except)
                                       (core:null loop)))
                                    (:if (mu:eq form :r)
                                         loop
                                         ((:lambda ()
                                            (core:format :t ";;; unrecognized debugger command: h for help~%" ())
                                            (core:null loop))))))))))
         ((:lambda ()
            (core:format :t "prelude:debug> " ())
            (mu:flush mu:*standard-output*)
            (core:read mu:*standard-input* () core:%eof%)))))
     ())))

;;;
;;; utilities
;;;
#|
(core:intern core::ns :extern "make-exception"
  (:lambda (obj condition source)
    (core:make-struct :except
      (core:cons obj
        (core:cons condition
          (core:cons source
            (core::list (core::frames))))))))

(core:intern core::ns :extern "print-exception"
  (:lambda (stream exception)
    ((:lambda (obj source condition)
       (core:format stream 
                    ";;; exception raised on ~A, condition ~A by ~A~%"
                    (core::list-3 obj source condition)))
     (core:vector-ref (core:struct-vec exception) 0)
     (core:vector-ref (core:struct-vec exception) 1)
     (core:vector-ref (core:struct-vec exception) 2))))
     
(core:intern core::ns :intern "print-frames"
  (:lambda (frames)
    (core:mapc
      (:lambda (frame)
        (core:format :t "(~A ~A)~%" frame))
        frames)))

(core:intern core::ns :intern "eprinc"
  (:lambda (obj)
    (core:write obj () core:*error-output*)))           

(core:intern core::ns :intern "eprint"
  (:lambda (obj)
    (core:write obj :t core:*error-output*)))           

(core:intern core::ns :intern "princ"
  (:lambda (obj)
    (core:write obj () core:*standard-output*)))           

(core:intern core::ns :intern "print"
  (:lambda (obj)
    (core:write obj :t core:*standard-output*)))           

(core:intern core::ns :extern "raise"
   (:lambda (obj msg)
     (core::eprinc "error: ")
     (core::eprinc msg)
     (core::eprinc " ")
     (core::eprint obj)
     (core:terpri core:*error-output*)
     (core:raise :error obj)))

(core:intern core::ns :extern "raise-if"
   (:lambda (test obj msg)
     (:if test
          ((:lambda ()
             (core::eprinc "raise-if: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri core:*error-output*)
             (core:raise :error obj)))
          obj)))

(core:intern core::ns :extern "raise-when"
   (:lambda (fn obj msg)
     (:if (core:apply fn (core::list obj))
          ((:lambda ()
             (core::eprinc "raise-when: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri core:*error-output*)
             (core:raise :error obj)))
          obj)))

(core:intern core::ns :extern "raise-unless"
   (:lambda (fnp obj msg)
     (:if (core:apply fnp (core::list obj))
          obj
          ((:lambda ()
             (core::eprinc "raise-unless: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri core:*error-output*)
             (core:raise :error obj))))))

(core:intern core::ns :extern "warn"
   (:lambda (obj msg)
     (core::eprinc "warning: ")
     (core::eprinc msg)
     (core::eprinc " ")
     (core::eprint obj)
     (core:terpri core:*error-output*)
     obj))

;;;
;;; break loop
;;;
(core:intern core::ns :extern "break"
   (:lambda (except)
     (core:raise-unless core:stringp except "core:break: not an exception")
     (core:write ";;; entering break loop, :h for commands" () ())
     (core:terpri ())
     (core:fix
      (:lambda (loop)
        (:if (core:stringp loop)
             loop
             (:if (core:eof :t)
                  ()
                  ((:lambda (form)
                     (:if (core:eq form :h)
                          ((:lambda ()
                             (core:write "break help:" () ()) (core:terpri ())
                             (core:write ":x - exit process" () ()) (core:terpri ())
                             (core:write ":r - return from break" () ()) (core:terpri ())
                             (core:write ":s - print stack frames" () ()) (core:terpri ())
                             (core:null loop)))
                          (:if (core:eq form :x)
                               (sys:exit 0)
                               (:if (core:eq form :s)
                                    ((:lambda ()
                                       (core:mapc
                                        (:lambda (frame) (core::print-frame frame))
                                        (core:car (core::context)))
                                       (core:null loop)))
                                    (:if (core:eq form :r)
                                         except
                                         ((:lambda ()
                                            (core:write (core:eval (core:compile form)) () :t)
                                            (core:terpri ())
                                            (core:null loop))))))))
                   ((:lambda ()
                      (core:write "break> " () ())
                      (core:read :t () ())))))))
     ())))
|#
