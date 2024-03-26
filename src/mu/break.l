;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; exceptions
;;;

;;;
;;; utilities
;;;
#|
(libcore:intern libcore::ns :extern "make-exception"
  (:lambda (obj condition source)
    (libcore:struct :except
      (libcore:cons obj
        (libcore:cons condition
          (libcore:cons source
            (libcore::list (libcore::frames))))))))

(libcore:intern libcore::ns :extern "print-exception"
  (:lambda (stream exception)
    ((:lambda (obj source condition)
       (libcore:format stream 
                    ";;; exception raised on ~A, condition ~A by ~A~%"
                    (libcore::list-3 obj source condition)))
     (libcore:sv-ref (libcore:st-vec exception) 0)
     (libcore:sv-ref (libcore:st-vec exception) 1)
     (libcore:sv-ref (libcore:st-vec exception) 2))))
     
(libcore:intern libcore::ns :intern "print-frames"
  (:lambda (frames)
    (libcore:mapc
      (:lambda (frame)
        (libcore:format :t "(~A ~A)~%" frame))
        frames)))

(libcore:intern libcore::ns :intern "eprinc"
  (:lambda (obj)
    (libcore:write obj () libcore:err-out)))           

(libcore:intern libcore::ns :intern "eprint"
  (:lambda (obj)
    (libcore:write obj :t libcore:err-out)))           

(libcore:intern libcore::ns :intern "princ"
  (:lambda (obj)
    (libcore:write obj () libcore:std-out)))           

(libcore:intern libcore::ns :intern "print"
  (:lambda (obj)
    (libcore:write obj :t libcore:std-out)))           

(libcore:intern libcore::ns :extern "raise"
   (:lambda (obj msg)
     (libcore::eprinc "error: ")
     (libcore::eprinc msg)
     (libcore::eprinc " ")
     (libcore::eprint obj)
     (libcore:terpri libcore:err-out)
     (libcore:raise :error obj)))

(libcore:intern libcore::ns :extern "raise-if"
   (:lambda (test obj msg)
     (:if test
          ((:lambda ()
             (libcore::eprinc "raise-if: ")
             (libcore::eprinc msg)
             (libcore::eprinc " ")
             (libcore::eprint obj)
             (libcore:terpri libcore:err-out)
             (libcore:raise :error obj)))
          obj)))

(libcore:intern libcore::ns :extern "raise-when"
   (:lambda (fn obj msg)
     (:if (libcore:apply fn (libcore::list obj))
          ((:lambda ()
             (libcore::eprinc "raise-when: ")
             (libcore::eprinc msg)
             (libcore::eprinc " ")
             (libcore::eprint obj)
             (libcore:terpri libcore:err-out)
             (libcore:raise :error obj)))
          obj)))

(libcore:intern libcore::ns :extern "raise-unless"
   (:lambda (fnp obj msg)
     (:if (libcore:apply fnp (libcore::list obj))
          obj
          ((:lambda ()
             (libcore::eprinc "raise-unless: ")
             (libcore::eprinc msg)
             (libcore::eprinc " ")
             (libcore::eprint obj)
             (libcore:terpri libcore:err-out)
             (libcore:raise :error obj))))))

(libcore:intern libcore::ns :extern "warn"
   (:lambda (obj msg)
     (libcore::eprinc "warning: ")
     (libcore::eprinc msg)
     (libcore::eprinc " ")
     (libcore::eprint obj)
     (libcore:terpri libcore:err-out)
     obj))

;;;
;;; break loop
;;;
(libcore:intern libcore::ns :extern "break"
   (:lambda (except)
     (libcore:raise-unless libcore:stringp except "libcore:break: not an exception")
     (libcore:write ";;; entering break loop, :h for commands" () ())
     (libcore:terpri ())
     (libcore:fix
      (:lambda (loop)
        (:if (libcore:stringp loop)
             loop
             (:if (libcore:eof :t)
                  ()
                  ((:lambda (form)
                     (:if (libcore:eq form :h)
                          ((:lambda ()
                             (libcore:write "break help:" () ()) (libcore:terpri ())
                             (libcore:write ":x - exit process" () ()) (libcore:terpri ())
                             (libcore:write ":r - return from break" () ()) (libcore:terpri ())
                             (libcore:write ":s - print stack frames" () ()) (libcore:terpri ())
                             (libcore:null loop)))
                          (:if (libcore:eq form :x)
                               (sys:exit 0)
                               (:if (libcore:eq form :s)
                                    ((:lambda ()
                                       (libcore:mapc
                                        (:lambda (frame) (libcore::print-frame frame))
                                        (libcore:car (libcore::context)))
                                       (libcore:null loop)))
                                    (:if (libcore:eq form :r)
                                         except
                                         ((:lambda ()
                                            (libcore:write (libcore:eval (libcore:compile form)) () :t)
                                            (libcore:terpri ())
                                            (libcore:null loop))))))))
                   ((:lambda ()
                      (libcore:write "break> " () ())
                      (libcore:read :t () ())))))))
     ())))
|#
