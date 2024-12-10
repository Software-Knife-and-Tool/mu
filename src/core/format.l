;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; format
;;;
(mu:intern core "%fmt-tilde"
  (:lambda (dest arg-list)
    (mu:write-char #\~ dest)
    arg-list))

(mu:intern core "%fmt-tab"
  (:lambda (dest arg-list)
    (mu:write-char #\tab dest)
    arg-list))

(mu:intern core "%fmt-percent"
  (:lambda (dest arg-list)
    (mu:write-char #\linefeed dest)
    arg-list))

(mu:intern core "%fmt-bang"
  (:lambda (dest arg-list)
    (mu:cdr arg-list)))

(mu:intern core "%fmt-decimal"
  (:lambda (dest arg-list)
     (:if (core:fixnump (mu:car arg-list))
          (core:%prog2
              (core:write (mu:car arg-list) () dest)
              (mu:cdr arg-list))
          (core:raise (mu:car arg-list) 'core:%format "argument to ~D not a fixnum"))))

(mu:intern core "%fmt-hex"
  (:lambda (dest arg-list)
     ((:lambda (fx)
       (:if (core:fixnump fx)
            (:if (mu:eq 0 fx)
                 (core:write "0" () dest)
                 ((:lambda (str-stream)
                     (mu:fix
                      (:lambda (n)
                         (:if (core:logor (core:null n) (mu:eq 0 n))
                              ()
                              (core:%prog2
                                  (mu:write (mu:svref "0123456789abcdef" (mu:logand n #xf)) () str-stream)
                                  (mu:div n 16))))
                      (:if (mu:less-than fx 0) (mu:mul -1 fx) fx))
                     (:if (mu:less-than fx 0)
                          (mu:write #\- () str-stream)
                          ())
                     ((:lambda (hexstr)
                         (mu:fix
                          (:lambda (len)
                             (:if (mu:eq 0 len)
                                  0
                                  (core:%prog2
                                     (mu:write (mu:svref hexstr (mu:sub len 1)) () dest)
                                     (mu:sub len 1))))
                          (mu:vector-length hexstr)))
                      (mu:get-string str-stream)))
                  (mu:open :string :output "" :t)))
            (core:raise fx 'core:%format "argument to ~X not a fixnum")))
      (mu:car arg-list))))

(mu:intern core "%fmt-aesthetic"
  (:lambda (dest arg-list)
    (core:write (mu:car arg-list) () dest)
    (mu:cdr arg-list)))

(mu:intern core "%fmt-standard"
  (:lambda (dest arg-list)
    (core:write (mu:car arg-list) :t dest)
    (mu:cdr arg-list)))

;;;
;;; controls
;;;
(mu:intern core "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (core:apply (mu:symbol-value fn) (core:%list2 dest arg-list))
            (core:raise dir 'core:%format "unrecognized format directive")))
     (mu:cdr
      (core:%assq
       dir
       '((#\A . core:%fmt-aesthetic)
         (#\S . core:%fmt-standard)
         (#\X . core:%fmt-hex)
         (#\D . core:%fmt-decimal)
         (#\T . core:%fmt-tab)
         (#\~ . core:%fmt-tilde)
         (#\% . core:%fmt-percent)
         (#\! . core:%fmt-bang)))))))

(mu:intern core "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (mu:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (mu:eq nth (mu:vector-length fmt))
               loop
               ((:lambda (ch)
                  (:if (mu:eq ch #\~)
                       (mu:cons (mu:add 2 nth) (core:%fmt-op (mu:svref fmt (mu:add 1 nth)) dest argl))
                       (core:%prog2
                           (mu:write-char ch dest)
                           (mu:cons (mu:add 1 nth) argl))))
                (mu:svref fmt nth))))
        (mu:car loop)
        (mu:cdr loop)))
     (mu:cons 0 arg-list))
    (:if (core:null stream)
         (mu:get-string dest)
         ())))

(mu:intern core "%format"
   (:lambda (stream format-string arg-list)
      (:if (core:stringp format-string)
           (:if (core:listp arg-list)
                (:if (mu:eq 0 (mu:vector-length format-string))
                     (:if stream
                          ""
                          ())
                     ((:lambda (dest) (core:%fmt-loop stream dest format-string arg-list))
                      (:if (core:null stream)
                           (mu:open :string :output "" :t)
                           (:if (mu:eq stream :t)
                                mu:*standard-output*
                                (:if (core:streamp stream)
                                     stream
                                     (core:raise stream 'core:%format "not a stream designator"))))))
                (core:raise arg-list 'core:%format "not a list"))
           (core:raise fmt 'core:%format "not a format string"))))
