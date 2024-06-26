;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; parsers
;;;

;;;
;;; number reader
;;;
(mu:intern prelude "parse-integer"
  (:lambda (digits base)
    (:if (prelude:stringp digits)
         (:if (prelude:fixnump base)
              (:if (prelude:zerop (mu:vector-len digits))
                   ()
                   ((:lambda (sign)
                       ((:lambda (integer)
                           (:if (prelude:fixnump integer)
                                (:if (prelude:null sign)
                                     integer
                                     (mu:product sign integer))
                                ()))
                        (mu:cdr
                         (mu:fix
                          (:lambda (loop)
                             (:if (prelude:null loop)
                                  ()
                                  ((:lambda (index acc)
                                      (:if (mu:less-than (mu:difference (mu:vector-len digits) 1) index)
                                           loop
                                           ((:lambda (n)
                                               (:if (prelude:null n)
                                                    ()
                                                    (:if (mu:less-than (mu:difference base 1) n)
                                                         ()
                                                         `(,(mu:sum 1 index) ,@(mu:sum n (mu:product acc base))))))
                                            (prelude:string-position (prelude:schar digits index) "0123456789abcdef"))))
                                   (mu:car loop)
                                   (mu:cdr loop))))
                          (:if (prelude:fixnump sign)
                               '(1 . 0)
                               '(0 . 0))))))
                    ((:lambda (digit)
                        (:if (mu:eq #\- digit)
                             -1
                             (:if (mu:eq #\+ digit)
                                  1
                                  ())))
                     (prelude:schar digits 0))))
              (prelude:raise base 'prelude:parse-integer "not a fixnum"))
         (prelude:raise digits 'prelude:parse-integer "not a string"))))

(mu:intern prelude "parse-float"
  (:lambda (str)
    ((:lambda (stream)
       ((:lambda (float)
          (:if (prelude:floatp float)
               float
               ()))
          (mu:read stream () ())))
       (mu:open :string :input str))))
