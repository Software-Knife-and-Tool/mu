;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; debugging
;;;
(libcore:intern :prelude "%debug-map-symbol"
   (:lambda (value)
      (:if (libcore:boundp 'prelude:%debug-symbols%)
           ()
           (libcore:intern :prelude "%debug-symbols%" (libcore:ns-syms :list :prelude)))
      (libcore:fix
       (:lambda (list)
          (:if (prelude:consp list)
               ((:lambda (symbol)
                   (:if (libcore:eq (libcore:sy-val symbol) value)
                        symbol
                        (libcore:cdr list)))
                (libcore:car list))
               list))
       prelude:%debug-symbols%)))

(libcore:intern :prelude "%format-mapped-symbol"
   (:lambda (value)
     ((:lambda (symbol)
        (:if (prelude:null symbol)
             value
             symbol))
       (prelude:%debug-map-symbol value))))

;;;
;;; break loop
;;;
(libcore:intern :prelude "break"
   (:lambda (except)
     (prelude:format :t ";;; entering break loop with exception~%" ())
     (prelude:%exceptf :t ";;; ~A on ~A by ~S, ~A~%" () except)
     (prelude:format :t ";;; :h for commands~%" ())
     (libcore:fix
      (:lambda (loop)
        ((:lambda (form)
           (:if (libcore:eq form prelude:%eof%)
                loop
                (:if (libcore:eq form :h)
                     ((:lambda ()
                        (prelude:format :t "break help:~%" ())
                        (prelude:format :t ":d - describe exception value~%" ())
                        (prelude:format :t ":e - print the full exception~%" ())
                        (prelude:format :t ":r - return from break~%" ())
                        (prelude:format :t ":x - exit process~%" ())
                        (prelude:null loop)))
                     (:if (libcore:eq form :d)
                          ((:lambda (value)
                             (prelude:describe (libcore:cdr value) :t)
                             (prelude:null loop))
                           (prelude:%except-prop :value except))
                          (:if (libcore:eq form :x)
                               (libcore:exit 0)
                               (:if (libcore:eq form :e)
                                    ((:lambda ()
                                       (prelude:%exceptf libcore:std-out "exception: ~A on ~A by ~S, ~A~%" :t except)
                                       (prelude:null loop)))
                                    (:if (libcore:eq form :r)
                                         loop
                                         ((:lambda ()
                                            (prelude:format :t ";;; unrecognized debugger command: h for help~%" ())
                                            (prelude:null loop))))))))))
         ((:lambda ()
            (prelude:format :t "prelude:debug> " ())
            (libcore:flush libcore:std-out)
            (prelude:read :t () prelude:%eof%)))))
     ())))
