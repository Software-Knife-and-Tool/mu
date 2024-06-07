;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; format
;;;
(mu:intern prelude "%fmt-tilde"
  (:lambda (dest arg-list)
    (prelude:write-char #\~ dest)
    arg-list))

(mu:intern prelude "%fmt-tab"
  (:lambda (dest arg-list)
    (prelude:write-char #\tab dest)
    arg-list))

(mu:intern prelude "%fmt-percent"
  (:lambda (dest arg-list)
    (prelude:write-char #\linefeed dest)
    arg-list))

(mu:intern prelude "%fmt-bang"
  (:lambda (dest arg-list)
    (mu:cdr arg-list)))

(mu:intern prelude "%fmt-decimal"
  (:lambda (dest arg-list)
     (:if (prelude:fixnump (mu:car arg-list))
          ((:lambda ()
              (prelude:write (mu:car arg-list) () dest)
              (mu:cdr arg-list)))
          (prelude:raise (mu:car arg-list) 'prelude:format "argument to ~D not a fixnum"))))

(mu:intern prelude "%fmt-hex"
  (:lambda (dest arg-list)
     ((:lambda (fx)
       (:if (prelude:fixnump fx)
            (:if (prelude:zerop fx)
                 (prelude:write "0" () dest)
                 ((:lambda (str-stream)
                     (mu:fix
                      (:lambda (n)
                         (:if (prelude:%orf (prelude:null n) (mu:eq 0 n))
                              ()
                              ((:lambda ()
                                  (mu:write (prelude:schar "0123456789abcdef" (mu:logand n #xf)) () str-stream)
                                  (mu:quotient n 16)))))
                      (:if (mu:less-than fx 0) (mu:product -1 fx) fx))
                     (:if (mu:less-than fx 0)
                          (mu:write #\- () str-stream)
                          ())
                     ((:lambda (hexstr)
                         (mu:fix
                          (:lambda (len)
                             (:if (prelude:zerop len)
                          0
                          ((:lambda ()
                              (mu:write (prelude:schar hexstr (mu:difference len 1)) () dest)
                              (mu:difference len 1)))))
                   (mu:vector-len hexstr)))
                  (mu:get-string str-stream)))
             (mu:open :string :output "")))
            (prelude:raise fx 'prelude:format "argument to ~X not a fixnum")))
      (mu:car arg-list))))

(mu:intern prelude "%fmt-aesthetic"
  (:lambda (dest arg-list)
    (prelude:write (mu:car arg-list) () dest)
    (mu:cdr arg-list)))

(mu:intern prelude "%fmt-standard"
  (:lambda (dest arg-list)
    (prelude:write (mu:car arg-list) :t dest)
    (mu:cdr arg-list)))

;;;
;;; control ops
;;;
(mu:intern prelude "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (prelude:apply (mu:symbol-value fn) `(,dest ,arg-list))
            (prelude:raise dir 'prelude:format "unrecognized format directive")))
     (mu:cdr
      (prelude:%assq
       dir
       '((#\A . prelude:%fmt-aesthetic)
         (#\S . prelude:%fmt-standard)
         (#\X . prelude:%fmt-hex)
         (#\D . prelude:%fmt-decimal)
         (#\T . prelude:%fmt-tab)
         (#\~ . prelude:%fmt-tilde)
         (#\% . prelude:%fmt-percent)
         (#\! . prelude:%fmt-bang)))))))

(mu:intern prelude "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (mu:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (mu:eq nth (mu:vector-len fmt))
               loop
               ((:lambda (ch)
                  (:if (mu:eq ch #\~)
                       `(,(mu:sum 2 nth) ,@(prelude:%fmt-op (prelude:schar fmt (mu:sum 1 nth)) dest argl))
                       ((:lambda ()
                           (prelude:write-char ch dest)
                           `(,(mu:sum 1 nth) ,@argl)))))
                (prelude:schar fmt nth))))
        (mu:car loop)
        (mu:cdr loop)))
     `(0 ,@arg-list))
    (:if (prelude:null stream)
         (prelude:get-output-stream-string dest)
         ())))

(mu:intern prelude "format"
   (:lambda (stream format-string arg-list)
      (:if (prelude:stringp format-string)
           (:if (prelude:listp arg-list)
                (:if (prelude:zerop (mu:vector-len format-string))
                     (:if stream
                          ""
                          ())
                     ((:lambda (dest) (prelude:%fmt-loop stream dest format-string arg-list))
                      (:if (prelude:null stream)
                           (mu:open :string :output "")
                           (:if (mu:eq stream :t)
                                mu:*standard-output*
                                (:if (prelude:streamp stream)
                                     stream
                                     (prelude:raise stream 'prelude:format "not a stream designator"))))))
                (prelude:raise arg-list 'prelude:format "not a list"))
           (prelude:raise fmt 'prelude:format "not a format string"))))
