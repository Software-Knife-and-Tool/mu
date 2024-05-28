;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; parsers
;;;

;;;
;;; number reader
;;;
(crux:intern prelude "parse-integer"
  (:lambda (digits base)
    (:if (prelude:stringp digits)
         (:if (prelude:fixnump base)
              (:if (prelude:zerop (crux:vector-len digits))
                   ()
                   ((:lambda (sign)
                       ((:lambda (integer)
                           (:if (prelude:fixnump integer)
                                (:if (prelude:null sign)
                                     integer
                                     (crux:product sign integer))
                                ()))
                        (crux:cdr
                         (crux:fix
                          (:lambda (loop)
                             (:if (prelude:null loop)
                                  ()
                                  ((:lambda (index acc)
                                      (:if (crux:less-than (crux:difference (crux:vector-len digits) 1) index)
                                           loop
                                           ((:lambda (n)
                                               (:if (prelude:null n)
                                                    ()
                                                    (:if (crux:less-than (crux:difference base 1) n)
                                                         ()
                                                         `(,(crux:sum 1 index) ,@(crux:sum n (crux:product acc base))))))
                                            (prelude:string-position (prelude:schar digits index) "0123456789abcdef"))))
                                   (crux:car loop)
                                   (crux:cdr loop))))
                          (:if (prelude:fixnump sign)
                               '(1 . 0)
                               '(0 . 0))))))
                    ((:lambda (digit)
                        (:if (crux:eq #\- digit)
                             -1
                             (:if (crux:eq #\+ digit)
                                  1
                                  ())))
                     (prelude:schar digits 0))))
              (prelude:raise base 'prelude:parse-integer "not a fixnum"))
         (prelude:raise digits 'prelude:parse-integer "not a string"))))

(crux:intern prelude "parse-float"
  (:lambda (str)
    ((:lambda (stream)
       ((:lambda (float)
          (:if (prelude:floatp float)
               float
               ()))
          (crux:read stream () ())))
       (crux:open :string :input str))))
