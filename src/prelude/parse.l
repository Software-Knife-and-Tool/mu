;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; parsers
;;;

;;;
;;; number reader
;;;
(core:intern prelude "parse-integer"
  (:lambda (digits base)
    (:if (prelude:stringp digits)
         (:if (prelude:fixnump base)
              (:if (prelude:zerop (core:vector-len digits))
                   ()
                   ((:lambda (sign)
                       ((:lambda (integer)
                           (:if (prelude:fixnump integer)
                                (:if (prelude:null sign)
                                     integer
                                     (core:product sign integer))
                                ()))
                        (core:cdr
                         (core:fix
                          (:lambda (loop)
                             (:if (prelude:null loop)
                                  ()
                                  ((:lambda (index acc)
                                      (:if (core:less-than (core:difference (core:vector-len digits) 1) index)
                                           loop
                                           ((:lambda (n)
                                               (:if (prelude:null n)
                                                    ()
                                                    (:if (core:less-than (core:difference base 1) n)
                                                         ()
                                                         `(,(core:sum 1 index) ,@(core:sum n (core:product acc base))))))
                                            (prelude:string-position (prelude:schar digits index) "0123456789abcdef"))))
                                   (core:car loop)
                                   (core:cdr loop))))
                          (:if (prelude:fixnump sign)
                               '(1 . 0)
                               '(0 . 0))))))
                    ((:lambda (digit)
                        (:if (core:eq #\- digit)
                             -1
                             (:if (core:eq #\+ digit)
                                  1
                                  ())))
                     (prelude:schar digits 0))))
              (prelude:raise base 'prelude:parse-integer "not a fixnum"))
         (prelude:raise digits 'prelude:parse-integer "not a string"))))

(core:intern prelude "parse-float"
  (:lambda (str)
    ((:lambda (stream)
       ((:lambda (float)
          (:if (prelude:floatp float)
               float
               ()))
          (core:read stream () ())))
       (core:open :string :input str))))
