;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; parsers
;;;

;;;
;;; number readers
;;;
(mu:intern core "%parse-integer"
  (:lambda (digits base)
    (:if (mu:eq 0 (mu:vector-length digits))
         ()
         ((:lambda (sign)
            ((:lambda (integer)
               (:if (core:fixnump integer)
                    (:if (core:null sign)
                         integer
                         (mu:mul sign integer))
                    ()))
             (mu:cdr
              (mu:fix
               (:lambda (loop)
                 (:if (core:null loop)
                      ()
                      ((:lambda (index acc)
                         (:if (mu:less-than (mu:sub (mu:vector-length digits) 1) index)
                              loop
                              ((:lambda (n)
                                 (:if (core:null n)
                                      ()
                                      (:if (mu:less-than (mu:sub base 1) n)
                                           ()
                                           ( mu:cons (mu:add 1 index) (mu:add n (mu:mul acc base))))))
                               (core:%string-position (mu:svref digits index) "0123456789abcdef"))))
                       (mu:car loop)
                       (mu:cdr loop))))
               (:if (core:fixnump sign)
                    '(1 . 0)
                    '(0 . 0))))))
          ((:lambda (digit)
             (:if (mu:eq #\- digit)
                  -1
                  (:if (mu:eq #\+ digit)
                       1
                       ())))
           (mu:svref digits 0))))))
    
(mu:intern core "%parse-float"
  (:lambda (str)
    ((:lambda (stream)
       ((:lambda (float)
          (:if (core:floatp float)
               float
               ()))
          (mu:read stream () ())))
       (mu:open :string :input str :t))))
