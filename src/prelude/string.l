;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(libcore:intern :prelude "stringp"
   (:lambda (string)
      (:if (libcore:eq (libcore:type-of string) :vector)
           (libcore:eq :char (libcore:sv-type string))
           (:if (prelude:%prelude-type-p string)
                (:if (libcore:eq "vector" (libcore:sv-ref (libcore:st-vec string) 0))
                     (libcore:eq :char (libcore:sv-type (prelude:%vector-prop :base string)))
                     ())
                ()))))

(libcore:intern :prelude "get-output-stream-string"
   (:lambda (stream)
      (libcore:get-str stream)))

(libcore:intern :prelude "schar"
  (:lambda (str index)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump index)
               (libcore:sv-ref str index)
               (prelude:raise index 'prelude:schar "not a string"))
          (prelude:raise str 'prelude:schar "not a string"))))

(libcore:intern :prelude "string="
   (:lambda (str-1 str-2)
      (:if (prelude:stringp str-1)
           (:if (prelude:stringp str-2)
                (:if (libcore:eq str-1 str-2)
                     :t
                     (:if (libcore:eq (libcore:sv-len str-1) (libcore:sv-len str-2))
                          (libcore:fix
                           (:lambda (nth)
                              (:if (prelude:numberp nth)
                                   (:if (libcore:eq nth (libcore:sv-len str-1))
                                        :t
                                        (:if (libcore:eq (libcore:sv-ref str-1 nth) (libcore:sv-ref str-2 nth))
                                             (libcore:fx-add 1 nth)
                                             ()))
                                   nth))
                           0)
                          ()))
                (prelude:raise str-2 'prelude:string= "not a string"))
           (prelude:raise str-1 'prelude:string= "not a string"))))

(libcore:intern :prelude "%string-write"
   (:lambda (string escape stream)
      (:if (libcore:eq :vector (libcore:type-of string))
           (libcore:write string escape stream)
           ((:lambda (length)
               (:if escape
                    (libcore:write #\" () stream)
                    ())
               (libcore:fix
                (:lambda (index)
                   (:if (libcore:fx-lt index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref string index) () stream)
                            (prelude:1+ index)))
                        index))
                0)
               (:if escape
                    (libcore:write #\" () stream)
                    ()))
            (prelude:%vector-prop :length string)))
      ()))

;;;
;;; string-position
;;;
(libcore:intern :prelude "string-position"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (libcore:sv-len str))
                   ()
                   ((:lambda (len)
                       (libcore:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (libcore:fx-lt (libcore:fx-sub len 1) loop)
                                     ()
                                     (:if (libcore:eq ch (libcore:sv-ref str loop))
                                          loop
                                          (libcore:fx-add 1 loop)))
                                loop))
                        0))
                    (libcore:sv-len str)))
              (prelude:raise str 'prelude:string-position "not a string"))
         (prelude:raise ch 'prelude:string-position "not a char"))))

;;;
;;; string-find
;;;
(libcore:intern :prelude "string-find"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (libcore:sv-len str))
                   ()
                   ((:lambda (len)
                       (libcore:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (libcore:fx-lt (libcore:fx-sub len 1) loop)
                                     ()
                                     (:if (libcore:eq ch (libcore:sv-ref str loop))
                                          ch
                                          (libcore:fx-add 1 loop)))
                                loop))
                        0))
                    (libcore:sv-len str)))
              (prelude:raise str 'prelude:string-find "not a string"))
         (prelude:raise ch 'prelude:string-find "not a char"))))

;;;
;;; string construction
;;;
(libcore:intern :prelude "string"
   (:lambda (designator)
      (:if (prelude:stringp designator)
         designator
         (:if (libcore:eq :symbol (libcore:type-of designator))
              (libcore:sy-name designator)
              (:if (prelude:charp designator)
                   (libcore:vector :char `(,designator))
                   (prelude:raise designator 'prelude:string "not a string designator"))))))

(libcore:intern :prelude "string-append"
   (:lambda (list)
      (:if (prelude:listp list)
           ((:lambda (stream)
               (libcore:fix
                (:lambda (list)
                   (:if list
                        ((:lambda ()
                            ((:lambda (str)
                                (:if (prelude:stringp str)
                                     (libcore:write str () stream)
                                     (prelude:raise str 'prelude:string-append "is not a string")))
                             (libcore:car list))
                            (libcore:cdr list)))
                        ()))
                list)
               (libcore:get-str stream))
            (libcore:open :string :output ""))
           (prelude:raise list 'prelude:string "is not a list"))))

(libcore:intern :prelude "substr"
  (:lambda (str start end)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump start)
               (:if (prelude:fixnump end)
                    ((:lambda (substr)
                        (libcore:fix
                         (:lambda (nth)
                            (:if (prelude:numberp nth)
                                 (:if (libcore:eq nth (libcore:sv-len str))
                                      ()
                                      (:if (libcore:fx-lt nth (libcore:fx-add 1 end))
                                           ((:lambda ()
                                               (libcore:write (libcore:sv-ref str nth) () substr)
                                               (libcore:fx-add 1 nth)))
                                           ()))
                                 nth))
                         start)
                        (libcore:get-str substr))
                     (libcore:open :string :output ""))
                    (prelude:raise end 'prelude:substr "end is not a fixnum"))
               (prelude:raise str 'prelude:substr "is not a string"))
          (prelude:raise start 'prelude:substr "start is not a fixnum"))))

#|
(libcore:intern :prelude "substr-"
  (:lambda (str start end)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump start)
               (:if (prelude:fixnump end)
                    (prelude:slice str start end)
                    (prelude:raise end 'prelude:substr "is not a fixnum"))
               (prelude:raise str 'prelude:substr "is not a string"))
          (prelude:raise start 'prelude:substr "is not a fixnum"))))
|#

;;;
;;; read line
;;;
(libcore:intern :prelude "read-line"
   (:lambda (stream eof-error eof-value)
      (:if (prelude:streamp stream)
           ((:lambda (sstream)
               (libcore:fix
                (:lambda (loop)
                   (:if (prelude:stringp loop)
                        loop
                        ((:lambda (ch)
                            (:if (libcore:eq ch #\linefeed)
                                 (libcore:get-str sstream)
                                 ((:lambda ()
                                     (libcore:wr-char ch sstream)
                                     (prelude:null loop)))))
                         (prelude:read-char stream eof-error eof-value))))
                ()))
            (libcore:open :string :output ""))
           (prelude:raise str 'prelude:read-line "is not a stream"))))

;;;
;;; read from string
;;;
(libcore:intern :prelude "%read-string%" (libcore:open :string :bidir ""))
(libcore:intern :prelude "read-string"
   (:lambda (str eof-error eof-value)
      (:if (prelude:stringp str)
           ((:lambda ()
              (libcore:write str () prelude:%read-string%) 
              (libcore:read prelude:%read-string% eof-error eof-value)))
           (prelude:raise str 'prelude:read-string "is not a string"))))

;;;
;;; write string
;;;
(libcore:intern :prelude "write-string"
   (:lambda (str designator)
      (:if (prelude:stringp str)
           ((:lambda (stream)
              (libcore:write str () stream))
            (prelude:%read-stream-designator designator))
           (prelude:raise str 'prelude:write-string "is not a string"))))

;;;
;;; write line
;;;
(libcore:intern :prelude "write-line"
   (:lambda (str designator)
      ((:lambda (stream)          
         (prelude:write-string str stream)
         (prelude:write-char #\linefeed stream))
       (prelude:%write-stream-designator designator))))
