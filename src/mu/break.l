;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;

;;;
;;; utilities
;;;
#|
(lib:intern lib::ns :extern "make-exception"
  (:lambda (obj condition source)
    (lib:make-struct :except
      (lib:cons obj
        (lib:cons condition
          (lib:cons source
            (lib::list (lib::frames))))))))

(lib:intern lib::ns :extern "print-exception"
  (:lambda (stream exception)
    ((:lambda (obj source condition)
       (lib:format stream 
                    ";;; exception raised on ~A, condition ~A by ~A~%"
                    (lib::list-3 obj source condition)))
     (lib:vector-ref (lib:struct-vec exception) 0)
     (lib:vector-ref (lib:struct-vec exception) 1)
     (lib:vector-ref (lib:struct-vec exception) 2))))
     
(lib:intern lib::ns :intern "print-frames"
  (:lambda (frames)
    (lib:mapc
      (:lambda (frame)
        (lib:format :t "(~A ~A)~%" frame))
        frames)))

(lib:intern lib::ns :intern "eprinc"
  (:lambda (obj)
    (lib:write obj () lib:error-output)))           

(lib:intern lib::ns :intern "eprint"
  (:lambda (obj)
    (lib:write obj :t lib:error-output)))           

(lib:intern lib::ns :intern "princ"
  (:lambda (obj)
    (lib:write obj () lib:standard-output)))           

(lib:intern lib::ns :intern "print"
  (:lambda (obj)
    (lib:write obj :t lib:standard-output)))           

(lib:intern lib::ns :extern "raise"
   (:lambda (obj msg)
     (lib::eprinc "error: ")
     (lib::eprinc msg)
     (lib::eprinc " ")
     (lib::eprint obj)
     (lib:terpri lib:error-output)
     (lib:raise :error obj)))

(lib:intern lib::ns :extern "raise-if"
   (:lambda (test obj msg)
     (:if test
          ((:lambda ()
             (lib::eprinc "raise-if: ")
             (lib::eprinc msg)
             (lib::eprinc " ")
             (lib::eprint obj)
             (lib:terpri lib:error-output)
             (lib:raise :error obj)))
          obj)))

(lib:intern lib::ns :extern "raise-when"
   (:lambda (fn obj msg)
     (:if (lib:apply fn (lib::list obj))
          ((:lambda ()
             (lib::eprinc "raise-when: ")
             (lib::eprinc msg)
             (lib::eprinc " ")
             (lib::eprint obj)
             (lib:terpri lib:error-output)
             (lib:raise :error obj)))
          obj)))

(lib:intern lib::ns :extern "raise-unless"
   (:lambda (fnp obj msg)
     (:if (lib:apply fnp (lib::list obj))
          obj
          ((:lambda ()
             (lib::eprinc "raise-unless: ")
             (lib::eprinc msg)
             (lib::eprinc " ")
             (lib::eprint obj)
             (lib:terpri lib:error-output)
             (lib:raise :error obj))))))

(lib:intern lib::ns :extern "warn"
   (:lambda (obj msg)
     (lib::eprinc "warning: ")
     (lib::eprinc msg)
     (lib::eprinc " ")
     (lib::eprint obj)
     (lib:terpri lib:error-output)
     obj))

;;;
;;; break loop
;;;
(lib:intern lib::ns :extern "break"
   (:lambda (except)
     (lib:raise-unless lib:stringp except "lib:break: not an exception")
     (lib:write ";;; entering break loop, :h for commands" () ())
     (lib:terpri ())
     (lib:fix
      (:lambda (loop)
        (:if (lib:stringp loop)
             loop
             (:if (lib:eof :t)
                  ()
                  ((:lambda (form)
                     (:if (lib:eq form :h)
                          ((:lambda ()
                             (lib:write "break help:" () ()) (lib:terpri ())
                             (lib:write ":x - exit process" () ()) (lib:terpri ())
                             (lib:write ":r - return from break" () ()) (lib:terpri ())
                             (lib:write ":s - print stack frames" () ()) (lib:terpri ())
                             (lib:null loop)))
                          (:if (lib:eq form :x)
                               (sys:exit 0)
                               (:if (lib:eq form :s)
                                    ((:lambda ()
                                       (lib:mapc
                                        (:lambda (frame) (lib::print-frame frame))
                                        (lib:car (lib::context)))
                                       (lib:null loop)))
                                    (:if (lib:eq form :r)
                                         except
                                         ((:lambda ()
                                            (lib:write (lib:eval (lib:compile form)) () :t)
                                            (lib:terpri ())
                                            (lib:null loop))))))))
                   ((:lambda ()
                      (lib:write "break> " () ())
                      (lib:read :t () ())))))))
     ())))
|#
