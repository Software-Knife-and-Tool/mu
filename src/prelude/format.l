;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; format
;;;
(core:intern prelude "%fmt-tilde"
  (:lambda (dest arg-list)
    (prelude:write-char #\~ dest)
    arg-list))

(core:intern prelude "%fmt-tab"
  (:lambda (dest arg-list)
    (prelude:write-char #\tab dest)
    arg-list))

(core:intern prelude "%fmt-percent"
  (:lambda (dest arg-list)
    (prelude:write-char #\linefeed dest)
    arg-list))

(core:intern prelude "%fmt-bang"
  (:lambda (dest arg-list)
    (core:cdr arg-list)))

(core:intern prelude "%fmt-decimal"
  (:lambda (dest arg-list)
     (:if (prelude:fixnump (core:car arg-list))
          ((:lambda ()
              (prelude:write (core:car arg-list) () dest)
              (core:cdr arg-list)))
          (prelude:raise (core:car arg-list) 'prelude:format "argument to ~D not a fixnum"))))

(core:intern prelude "%fmt-hex"
  (:lambda (dest arg-list)
     ((:lambda (fx)
       (:if (prelude:fixnump fx)
            (:if (prelude:zerop fx)
                 (prelude:write "0" () dest)
                 ((:lambda (str-stream)
                     (core:fix
                      (:lambda (n)
                         (:if (prelude:%orf (prelude:null n) (core:eq 0 n))
                              ()
                              ((:lambda ()
                                  (core:write (prelude:schar "0123456789abcdef" (core:logand n #xf)) () str-stream)
                                  (core:quotient n 16)))))
                      (:if (core:less-than fx 0) (core:product -1 fx) fx))
                     (:if (core:less-than fx 0)
                          (core:write #\- () str-stream)
                          ())
                     ((:lambda (hexstr)
                         (core:fix
                          (:lambda (len)
                             (:if (prelude:zerop len)
                          0
                          ((:lambda ()
                              (core:write (prelude:schar hexstr (core:difference len 1)) () dest)
                              (core:difference len 1)))))
                   (core:vector-len hexstr)))
                  (core:get-string str-stream)))
             (core:open :string :output "")))
            (prelude:raise fx 'prelude:format "argument to ~X not a fixnum")))
      (core:car arg-list))))

(core:intern prelude "%fmt-aesthetic"
  (:lambda (dest arg-list)
    (prelude:write (core:car arg-list) () dest)
    (core:cdr arg-list)))

(core:intern prelude "%fmt-standard"
  (:lambda (dest arg-list)
    (prelude:write (core:car arg-list) :t dest)
    (core:cdr arg-list)))

;;;
;;; control ops
;;;
(core:intern prelude "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (prelude:apply (core:symbol-value fn) `(,dest ,arg-list))
            (prelude:raise dir 'prelude:format "unrecognized format directive")))
     (core:cdr
      (prelude:%assoc
       dir
       '((#\A . prelude:%fmt-aesthetic)
         (#\S . prelude:%fmt-standard)
         (#\X . prelude:%fmt-hex)
         (#\D . prelude:%fmt-decimal)
         (#\T . prelude:%fmt-tab)
         (#\~ . prelude:%fmt-tilde)
         (#\% . prelude:%fmt-percent)
         (#\! . prelude:%fmt-bang)))))))

(core:intern prelude "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (core:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (core:eq nth (core:vector-len fmt))
               loop
               ((:lambda (ch)
                  (:if (core:eq ch #\~)
                       `(,(core:sum 2 nth) ,@(prelude:%fmt-op (prelude:schar fmt (core:sum 1 nth)) dest argl))
                       ((:lambda ()
                           (prelude:write-char ch dest)
                           `(,(core:sum 1 nth) ,@argl)))))
                (prelude:schar fmt nth))))
        (core:car loop)
        (core:cdr loop)))
     `(0 ,@arg-list))
    (:if (prelude:null stream)
         (prelude:get-output-stream-string dest)
         ())))

(core:intern prelude "format"
   (:lambda (stream format-string arg-list)
      (:if (prelude:stringp format-string)
           (:if (prelude:listp arg-list)
                (:if (prelude:zerop (core:vector-len format-string))
                     (:if stream
                          ""
                          ())
                     ((:lambda (dest) (prelude:%fmt-loop stream dest format-string arg-list))
                      (:if (prelude:null stream)
                           (core:open :string :output "")
                           (:if (core:eq stream :t)
                                core:standard-output
                                (:if (prelude:streamp stream)
                                     stream
                                     (prelude:raise stream 'prelude:format "not a stream designator"))))))
                (prelude:raise arg-list 'prelude:format "not a list"))
           (prelude:raise fmt 'prelude:format "not a format string"))))
