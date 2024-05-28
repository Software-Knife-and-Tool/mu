;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; format
;;;
(crux:intern prelude "%fmt-tilde"
  (:lambda (dest arg-list)
    (prelude:write-char #\~ dest)
    arg-list))

(crux:intern prelude "%fmt-tab"
  (:lambda (dest arg-list)
    (prelude:write-char #\tab dest)
    arg-list))

(crux:intern prelude "%fmt-percent"
  (:lambda (dest arg-list)
    (prelude:write-char #\linefeed dest)
    arg-list))

(crux:intern prelude "%fmt-bang"
  (:lambda (dest arg-list)
    (crux:cdr arg-list)))

(crux:intern prelude "%fmt-decimal"
  (:lambda (dest arg-list)
     (:if (prelude:fixnump (crux:car arg-list))
          ((:lambda ()
              (prelude:write (crux:car arg-list) () dest)
              (crux:cdr arg-list)))
          (prelude:raise (crux:car arg-list) 'prelude:format "argument to ~D not a fixnum"))))

(crux:intern prelude "%fmt-hex"
  (:lambda (dest arg-list)
     ((:lambda (fx)
       (:if (prelude:fixnump fx)
            (:if (prelude:zerop fx)
                 (prelude:write "0" () dest)
                 ((:lambda (str-stream)
                     (crux:fix
                      (:lambda (n)
                         (:if (prelude:%orf (prelude:null n) (crux:eq 0 n))
                              ()
                              ((:lambda ()
                                  (crux:write (prelude:schar "0123456789abcdef" (crux:logand n #xf)) () str-stream)
                                  (crux:quotient n 16)))))
                      (:if (crux:less-than fx 0) (crux:product -1 fx) fx))
                     (:if (crux:less-than fx 0)
                          (crux:write #\- () str-stream)
                          ())
                     ((:lambda (hexstr)
                         (crux:fix
                          (:lambda (len)
                             (:if (prelude:zerop len)
                          0
                          ((:lambda ()
                              (crux:write (prelude:schar hexstr (crux:difference len 1)) () dest)
                              (crux:difference len 1)))))
                   (crux:vector-len hexstr)))
                  (crux:get-string str-stream)))
             (crux:open :string :output "")))
            (prelude:raise fx 'prelude:format "argument to ~X not a fixnum")))
      (crux:car arg-list))))

(crux:intern prelude "%fmt-aesthetic"
  (:lambda (dest arg-list)
    (prelude:write (crux:car arg-list) () dest)
    (crux:cdr arg-list)))

(crux:intern prelude "%fmt-standard"
  (:lambda (dest arg-list)
    (prelude:write (crux:car arg-list) :t dest)
    (crux:cdr arg-list)))

;;;
;;; control ops
;;;
(crux:intern prelude "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (prelude:apply (crux:symbol-value fn) `(,dest ,arg-list))
            (prelude:raise dir 'prelude:format "unrecognized format directive")))
     (crux:cdr
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

(crux:intern prelude "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (crux:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (crux:eq nth (crux:vector-len fmt))
               loop
               ((:lambda (ch)
                  (:if (crux:eq ch #\~)
                       `(,(crux:sum 2 nth) ,@(prelude:%fmt-op (prelude:schar fmt (crux:sum 1 nth)) dest argl))
                       ((:lambda ()
                           (prelude:write-char ch dest)
                           `(,(crux:sum 1 nth) ,@argl)))))
                (prelude:schar fmt nth))))
        (crux:car loop)
        (crux:cdr loop)))
     `(0 ,@arg-list))
    (:if (prelude:null stream)
         (prelude:get-output-stream-string dest)
         ())))

(crux:intern prelude "format"
   (:lambda (stream format-string arg-list)
      (:if (prelude:stringp format-string)
           (:if (prelude:listp arg-list)
                (:if (prelude:zerop (crux:vector-len format-string))
                     (:if stream
                          ""
                          ())
                     ((:lambda (dest) (prelude:%fmt-loop stream dest format-string arg-list))
                      (:if (prelude:null stream)
                           (crux:open :string :output "")
                           (:if (crux:eq stream :t)
                                crux:*standard-output*
                                (:if (prelude:streamp stream)
                                     stream
                                     (prelude:raise stream 'prelude:format "not a stream designator"))))))
                (prelude:raise arg-list 'prelude:format "not a list"))
           (prelude:raise fmt 'prelude:format "not a format string"))))
