;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;

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
    (core:write obj () core:error-output)))           

(core:intern core::ns :intern "eprint"
  (:lambda (obj)
    (core:write obj :t core:error-output)))           

(core:intern core::ns :intern "princ"
  (:lambda (obj)
    (core:write obj () core:standard-output)))           

(core:intern core::ns :intern "print"
  (:lambda (obj)
    (core:write obj :t core:standard-output)))           

(core:intern core::ns :extern "raise"
   (:lambda (obj msg)
     (core::eprinc "error: ")
     (core::eprinc msg)
     (core::eprinc " ")
     (core::eprint obj)
     (core:terpri core:error-output)
     (core:raise :error obj)))

(core:intern core::ns :extern "raise-if"
   (:lambda (test obj msg)
     (:if test
          ((:lambda ()
             (core::eprinc "raise-if: ")
             (core::eprinc msg)
             (core::eprinc " ")
             (core::eprint obj)
             (core:terpri core:error-output)
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
             (core:terpri core:error-output)
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
             (core:terpri core:error-output)
             (core:raise :error obj))))))

(core:intern core::ns :extern "warn"
   (:lambda (obj msg)
     (core::eprinc "warning: ")
     (core::eprinc msg)
     (core::eprinc " ")
     (core::eprint obj)
     (core:terpri core:error-output)
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
