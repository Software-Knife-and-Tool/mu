;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(core:intern prelude "stringp"
   (:lambda (string)
      (:if (core:eq (core:type-of string) :vector)
           (core:eq :char (core:vector-type string))
           (:if (prelude:%prelude-type-p string)
                (:if (core:eq "vector" (core:vector-ref (core:struct-vec string) 0))
                     (core:eq :char (core:vector-type (prelude:%vector-prop :base string)))
                     ())
                ()))))

(core:intern prelude "get-output-stream-string"
   (:lambda (stream)
      (core:get-string stream)))

(core:intern prelude "schar"
  (:lambda (str index)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump index)
               (core:vector-ref str index)
               (prelude:raise index 'prelude:schar "not a string"))
          (prelude:raise str 'prelude:schar "not a string"))))

(core:intern prelude "string="
   (:lambda (str-1 str-2)
      (:if (prelude:stringp str-1)
           (:if (prelude:stringp str-2)
                (:if (core:eq str-1 str-2)
                     :t
                     (:if (core:eq (core:vector-len str-1) (core:vector-len str-2))
                          (core:fix
                           (:lambda (nth)
                              (:if (prelude:numberp nth)
                                   (:if (core:eq nth (core:vector-len str-1))
                                        :t
                                        (:if (core:eq (core:vector-ref str-1 nth) (core:vector-ref str-2 nth))
                                             (core:sum 1 nth)
                                             ()))
                                   nth))
                           0)
                          ()))
                (prelude:raise str-2 'prelude:string= "not a string"))
           (prelude:raise str-1 'prelude:string= "not a string"))))

(core:intern prelude "%string-write"
   (:lambda (string escape stream)
      (:if (core:eq :vector (core:type-of string))
           (core:write string escape stream)
           ((:lambda (length)
               (:if escape
                    (core:write #\" () stream)
                    ())
               (core:fix
                (:lambda (index)
                   (:if (core:less-than index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref string index) () stream)
                            (prelude:1+ index)))
                        index))
                0)
               (:if escape
                    (core:write #\" () stream)
                    ()))
            (prelude:%vector-prop :length string)))
      ()))

;;;
;;; string-position
;;;
(core:intern prelude "string-position"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (core:vector-len str))
                   ()
                   ((:lambda (len)
                       (core:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (core:less-than (core:difference len 1) loop)
                                     ()
                                     (:if (core:eq ch (core:vector-ref str loop))
                                          loop
                                          (core:sum 1 loop)))
                                loop))
                        0))
                    (core:vector-len str)))
              (prelude:raise str 'prelude:string-position "not a string"))
         (prelude:raise ch 'prelude:string-position "not a char"))))

;;;
;;; string-find
;;;
(core:intern prelude "string-find"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (core:vector-len str))
                   ()
                   ((:lambda (len)
                       (core:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (core:less-than (core:difference len 1) loop)
                                     ()
                                     (:if (core:eq ch (core:vector-ref str loop))
                                          ch
                                          (core:sum 1 loop)))
                                loop))
                        0))
                    (core:vector-len str)))
              (prelude:raise str 'prelude:string-find "not a string"))
         (prelude:raise ch 'prelude:string-find "not a char"))))

;;;
;;; string construction
;;;
(core:intern prelude "string"
   (:lambda (designator)
      (:if (prelude:stringp designator)
         designator
         (:if (core:eq :symbol (core:type-of designator))
              (core:symbol-name designator)
              (:if (prelude:charp designator)
                   (core:make-vector :char `(,designator))
                   (prelude:raise designator 'prelude:string "not a string designator"))))))

(core:intern prelude "string-append"
   (:lambda (list)
      (:if (prelude:listp list)
           ((:lambda (stream)
               (core:fix
                (:lambda (list)
                   (:if list
                        ((:lambda ()
                            ((:lambda (str)
                                (:if (prelude:stringp str)
                                     (core:write str () stream)
                                     (prelude:raise str 'prelude:string-append "is not a string")))
                             (core:car list))
                            (core:cdr list)))
                        ()))
                list)
               (core:get-string stream))
            (core:open :string :output ""))
           (prelude:raise list 'prelude:string "is not a list"))))

(core:intern prelude "substr"
  (:lambda (str start end)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump start)
               (:if (prelude:fixnump end)
                    ((:lambda (substr)
                        (core:fix
                         (:lambda (nth)
                            (:if (prelude:numberp nth)
                                 (:if (core:eq nth (core:vector-len str))
                                      ()
                                      (:if (core:less-than nth (core:sum 1 end))
                                           ((:lambda ()
                                               (core:write (core:vector-ref str nth) () substr)
                                               (core:sum 1 nth)))
                                           ()))
                                 nth))
                         start)
                        (core:get-string substr))
                     (core:open :string :output ""))
                    (prelude:raise end 'prelude:substr "end is not a fixnum"))
               (prelude:raise str 'prelude:substr "is not a string"))
          (prelude:raise start 'prelude:substr "start is not a fixnum"))))

#|
(core:intern prelude "substr-"
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
(core:intern prelude "read-line"
   (:lambda (stream eof-error eof-value)
      (:if (prelude:streamp stream)
           ((:lambda (sstream)
               (core:fix
                (:lambda (loop)
                   (:if (prelude:stringp loop)
                        loop
                        ((:lambda (ch)
                            (:if (core:eq ch #\linefeed)
                                 (core:get-string sstream)
                                 ((:lambda ()
                                     (core:write-char ch sstream)
                                     (prelude:null loop)))))
                         (prelude:read-char stream eof-error eof-value))))
                ()))
            (core:open :string :output ""))
           (prelude:raise str 'prelude:read-line "is not a stream"))))

;;;
;;; read from string
;;;
(core:intern prelude "%read-string%" (core:open :string :bidir ""))
(core:intern prelude "read-string"
   (:lambda (str eof-error eof-value)
      (:if (prelude:stringp str)
           ((:lambda ()
              (core:write str () prelude:%read-string%) 
              (core:read prelude:%read-string% eof-error eof-value)))
           (prelude:raise str 'prelude:read-string "is not a string"))))

;;;
;;; write string
;;;
(core:intern prelude "write-string"
   (:lambda (str designator)
      (:if (prelude:stringp str)
           ((:lambda (stream)
              (core:write str () stream))
            (prelude:%read-stream-designator designator))
           (prelude:raise str 'prelude:write-string "is not a string"))))

;;;
;;; write line
;;;
(core:intern prelude "write-line"
   (:lambda (str designator)
      ((:lambda (stream)          
         (prelude:write-string str stream)
         (prelude:write-char #\linefeed stream))
       (prelude:%write-stream-designator designator))))
