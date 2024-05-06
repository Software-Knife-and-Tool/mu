;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; format
;;;
(lib:intern prelude "%fmt-tilde"
  (:lambda (dest arg-list)
    (prelude:write-char #\~ dest)
    arg-list))

(lib:intern prelude "%fmt-tab"
  (:lambda (dest arg-list)
    (prelude:write-char #\tab dest)
    arg-list))

(lib:intern prelude "%fmt-percent"
  (:lambda (dest arg-list)
    (prelude:write-char #\linefeed dest)
    arg-list))

(lib:intern prelude "%fmt-bang"
  (:lambda (dest arg-list)
    (lib:cdr arg-list)))

(lib:intern prelude "%fmt-decimal"
  (:lambda (dest arg-list)
     (:if (prelude:fixnump (lib:car arg-list))
          ((:lambda ()
              (prelude:write (lib:car arg-list) () dest)
              (lib:cdr arg-list)))
          (prelude:raise (lib:car arg-list) 'prelude:format "argument to ~D not a fixnum"))))

(lib:intern prelude "%fmt-hex"
  (:lambda (dest arg-list)
     ((:lambda (fx)
       (:if (prelude:fixnump fx)
            (:if (prelude:zerop fx)
                 (prelude:write "0" () dest)
                 ((:lambda (str-stream)
                     (lib:fix
                      (:lambda (n)
                         (:if (prelude:%orf (prelude:null n) (lib:eq 0 n))
                              ()
                              ((:lambda ()
                                  (lib:write (prelude:schar "0123456789abcdef" (lib:logand n #xf)) () str-stream)
                                  (lib:quotient n 16)))))
                      (:if (lib:less-than fx 0) (lib:product -1 fx) fx))
                     (:if (lib:less-than fx 0)
                          (lib:write #\- () str-stream)
                          ())
                     ((:lambda (hexstr)
                         (lib:fix
                          (:lambda (len)
                             (:if (prelude:zerop len)
                          0
                          ((:lambda ()
                              (lib:write (prelude:schar hexstr (lib:difference len 1)) () dest)
                              (lib:difference len 1)))))
                   (lib:vector-len hexstr)))
                  (lib:get-string str-stream)))
             (lib:open :string :output "")))
            (prelude:raise fx 'prelude:format "argument to ~X not a fixnum")))
      (lib:car arg-list))))

(lib:intern prelude "%fmt-aesthetic"
  (:lambda (dest arg-list)
    (prelude:write (lib:car arg-list) () dest)
    (lib:cdr arg-list)))

(lib:intern prelude "%fmt-standard"
  (:lambda (dest arg-list)
    (prelude:write (lib:car arg-list) :t dest)
    (lib:cdr arg-list)))

;;;
;;; control ops
;;;
(lib:intern prelude "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (prelude:apply (lib:symbol-value fn) `(,dest ,arg-list))
            (prelude:raise dir 'prelude:format "unrecognized format directive")))
     (lib:cdr
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

(lib:intern prelude "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (lib:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (lib:eq nth (lib:vector-len fmt))
               loop
               ((:lambda (ch)
                  (:if (lib:eq ch #\~)
                       `(,(lib:sum 2 nth) ,@(prelude:%fmt-op (prelude:schar fmt (lib:sum 1 nth)) dest argl))
                       ((:lambda ()
                           (prelude:write-char ch dest)
                           `(,(lib:sum 1 nth) ,@argl)))))
                (prelude:schar fmt nth))))
        (lib:car loop)
        (lib:cdr loop)))
     `(0 ,@arg-list))
    (:if (prelude:null stream)
         (prelude:get-output-stream-string dest)
         ())))

(lib:intern prelude "format"
   (:lambda (stream format-string arg-list)
      (:if (prelude:stringp format-string)
           (:if (prelude:listp arg-list)
                (:if (prelude:zerop (lib:vector-len format-string))
                     (:if stream
                          ""
                          ())
                     ((:lambda (dest) (prelude:%fmt-loop stream dest format-string arg-list))
                      (:if (prelude:null stream)
                           (lib:open :string :output "")
                           (:if (lib:eq stream :t)
                                lib:standard-output
                                (:if (prelude:streamp stream)
                                     stream
                                     (prelude:raise stream 'prelude:format "not a stream designator"))))))
                (prelude:raise arg-list 'prelude:format "not a list"))
           (prelude:raise fmt 'prelude:format "not a format string"))))
