;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; repl
;;;
(core:intern prelude "repl"
   (:lambda ()
     (core:fix
       (:lambda (loop)
         (prelude:with-exception
             (:lambda (ex)
               (prelude:break ex)
               (prelude:null loop))
           (:lambda ()
             (prelude:format :t "prelude> " ())
             (core:flush core:*standard-output*)
             ((:lambda (form)
                (:if (core:eq form prelude:%eof%)
                     loop
                     ((:lambda (value)
                        (prelude:format :t "[~A] ~S~%" `(,(prelude:type-of value) ,value))
                        (prelude:null loop))
                      (core:eval (prelude:compile form)))))
              (prelude:read :t () prelude:%eof%)))))
       ())))
