;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; format
;;;
(libcore:intern :prelude "%fmt-tilde"
  (:lambda (dest arg-list)
    (prelude:write-char #\~ dest)
    arg-list))

(libcore:intern :prelude "%fmt-tab"
  (:lambda (dest arg-list)
    (prelude:write-char #\tab dest)
    arg-list))

(libcore:intern :prelude "%fmt-percent"
  (:lambda (dest arg-list)
    (prelude:write-char #\linefeed dest)
    arg-list))

(libcore:intern :prelude "%fmt-bang"
  (:lambda (dest arg-list)
    (libcore:cdr arg-list)))

(libcore:intern :prelude "%fmt-decimal"
  (:lambda (dest arg-list)
     (:if (prelude:fixnump (libcore:car arg-list))
          ((:lambda ()
              (prelude:write (libcore:car arg-list) () dest)
              (libcore:cdr arg-list)))
          (prelude:raise (libcore:car arg-list) 'prelude:format "argument to ~D not a fixnum"))))

(libcore:intern :prelude "%fmt-hex"
  (:lambda (dest arg-list)
     ((:lambda (fx)
       (:if (prelude:fixnump fx)
            (:if (prelude:zerop fx)
                 (prelude:write "0" () dest)
                 ((:lambda (str-stream)
                     (libcore:fix
                      (:lambda (n)
                         (:if (prelude:%orf (prelude:null n) (libcore:eq 0 n))
                              ()
                              ((:lambda ()
                                  (libcore:write (prelude:schar "0123456789abcdef" (libcore:logand n #xf)) () str-stream)
                                  (libcore:fx-div n 16)))))
                      (:if (libcore:fx-lt fx 0) (libcore:fx-mul -1 fx) fx))
                     (:if (libcore:fx-lt fx 0)
                          (libcore:write #\- () str-stream)
                          ())
                     ((:lambda (hexstr)
                         (libcore:fix
                          (:lambda (len)
                             (:if (prelude:zerop len)
                          0
                          ((:lambda ()
                              (libcore:write (prelude:schar hexstr (libcore:fx-sub len 1)) () dest)
                              (libcore:fx-sub len 1)))))
                   (libcore:sv-len hexstr)))
                  (libcore:get-str str-stream)))
             (libcore:open :string :output "")))
            (prelude:raise fx 'prelude:format "argument to ~X not a fixnum")))
      (libcore:car arg-list))))

(libcore:intern :prelude "%fmt-aesthetic"
  (:lambda (dest arg-list)
    (prelude:write (libcore:car arg-list) () dest)
    (libcore:cdr arg-list)))

(libcore:intern :prelude "%fmt-standard"
  (:lambda (dest arg-list)
    (prelude:write (libcore:car arg-list) :t dest)
    (libcore:cdr arg-list)))

;;;
;;; control ops
;;;
(libcore:intern :prelude "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (prelude:apply (libcore:sy-val fn) `(,dest ,arg-list))
            (prelude:raise dir 'prelude:format "unrecognized format directive")))
     (libcore:cdr
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

(libcore:intern :prelude "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (libcore:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (libcore:eq nth (libcore:sv-len fmt))
               loop
               ((:lambda (ch)
                  (:if (libcore:eq ch #\~)
                       `(,(libcore:fx-add 2 nth) ,@(prelude:%fmt-op (prelude:schar fmt (libcore:fx-add 1 nth)) dest argl))
                       ((:lambda ()
                           (prelude:write-char ch dest)
                           `(,(libcore:fx-add 1 nth) ,@argl)))))
                (prelude:schar fmt nth))))
        (libcore:car loop)
        (libcore:cdr loop)))
     `(0 ,@arg-list))
    (:if (prelude:null stream)
         (prelude:get-output-stream-string dest)
         ())))

(libcore:intern :prelude "format"
   (:lambda (stream format-string arg-list)
      (:if (prelude:stringp format-string)
           (:if (prelude:listp arg-list)
                (:if (prelude:zerop (libcore:sv-len format-string))
                     (:if stream
                          ""
                          ())
                     ((:lambda (dest) (prelude:%fmt-loop stream dest format-string arg-list))
                      (:if (prelude:null stream)
                           (libcore:open :string :output "")
                           (:if (libcore:eq stream :t)
                                libcore:std-out
                                (:if (prelude:streamp stream)
                                     stream
                                     (prelude:raise stream 'prelude:format "not a stream designator"))))))
                (prelude:raise arg-list 'prelude:format "not a list"))
           (prelude:raise fmt 'prelude:format "not a format string"))))
