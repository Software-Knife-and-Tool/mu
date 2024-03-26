;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; parsers
;;;

;;;
;;; number reader
;;;
(libcore:intern :prelude "parse-integer"
  (:lambda (digits base)
    (:if (prelude:stringp digits)
         (:if (prelude:fixnump base)
              (:if (prelude:zerop (libcore:sv-len digits))
                   ()
                   ((:lambda (sign)
                       ((:lambda (integer)
                           (:if (prelude:fixnump integer)
                                (:if (prelude:null sign)
                                     integer
                                     (libcore:fx-mul sign integer))
                                ()))
                        (libcore:cdr
                         (libcore:fix
                          (:lambda (loop)
                             (:if (prelude:null loop)
                                  ()
                                  ((:lambda (index acc)
                                      (:if (libcore:fx-lt (libcore:fx-sub (libcore:sv-len digits) 1) index)
                                           loop
                                           ((:lambda (n)
                                               (:if (prelude:null n)
                                                    ()
                                                    (:if (libcore:fx-lt (libcore:fx-sub base 1) n)
                                                         ()
                                                         `(,(libcore:fx-add 1 index) ,@(libcore:fx-add n (libcore:fx-mul acc base))))))
                                            (prelude:string-position (prelude:schar digits index) "0123456789abcdef"))))
                                   (libcore:car loop)
                                   (libcore:cdr loop))))
                          (:if (prelude:fixnump sign)
                               '(1 . 0)
                               '(0 . 0))))))
                    ((:lambda (digit)
                        (:if (libcore:eq #\- digit)
                             -1
                             (:if (libcore:eq #\+ digit)
                                  1
                                  ())))
                     (prelude:schar digits 0))))
              (prelude:raise base 'prelude:parse-integer "not a fixnum"))
         (prelude:raise digits 'prelude:parse-integer "not a string"))))

(libcore:intern :prelude "parse-float"
  (:lambda (str)
    ((:lambda (stream)
       ((:lambda (float)
          (:if (prelude:floatp float)
               float
               ()))
          (libcore:read stream () ())))
       (libcore:open :string :input str))))
