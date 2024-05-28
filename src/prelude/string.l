;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(crux:intern prelude "stringp"
   (:lambda (string)
      (:if (crux:eq (crux:type-of string) :vector)
           (crux:eq :char (crux:vector-type string))
           (:if (prelude:%prelude-type-p string)
                (:if (crux:eq "vector" (crux:vector-ref (crux:struct-vec string) 0))
                     (crux:eq :char (crux:vector-type (prelude:%vector-prop :base string)))
                     ())
                ()))))

(crux:intern prelude "get-output-stream-string"
   (:lambda (stream)
      (crux:get-string stream)))

(crux:intern prelude "schar"
  (:lambda (str index)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump index)
               (crux:vector-ref str index)
               (prelude:raise index 'prelude:schar "not a string"))
          (prelude:raise str 'prelude:schar "not a string"))))

(crux:intern prelude "string="
   (:lambda (str-1 str-2)
      (:if (prelude:stringp str-1)
           (:if (prelude:stringp str-2)
                (:if (crux:eq str-1 str-2)
                     :t
                     (:if (crux:eq (crux:vector-len str-1) (crux:vector-len str-2))
                          (crux:fix
                           (:lambda (nth)
                              (:if (prelude:numberp nth)
                                   (:if (crux:eq nth (crux:vector-len str-1))
                                        :t
                                        (:if (crux:eq (crux:vector-ref str-1 nth) (crux:vector-ref str-2 nth))
                                             (crux:sum 1 nth)
                                             ()))
                                   nth))
                           0)
                          ()))
                (prelude:raise str-2 'prelude:string= "not a string"))
           (prelude:raise str-1 'prelude:string= "not a string"))))

(crux:intern prelude "%string-write"
   (:lambda (string escape stream)
      (:if (crux:eq :vector (crux:type-of string))
           (crux:write string escape stream)
           ((:lambda (length)
               (:if escape
                    (crux:write #\" () stream)
                    ())
               (crux:fix
                (:lambda (index)
                   (:if (crux:less-than index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref string index) () stream)
                            (prelude:1+ index)))
                        index))
                0)
               (:if escape
                    (crux:write #\" () stream)
                    ()))
            (prelude:%vector-prop :length string)))
      ()))

;;;
;;; string-position
;;;
(crux:intern prelude "string-position"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (crux:vector-len str))
                   ()
                   ((:lambda (len)
                       (crux:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (crux:less-than (crux:difference len 1) loop)
                                     ()
                                     (:if (crux:eq ch (crux:vector-ref str loop))
                                          loop
                                          (crux:sum 1 loop)))
                                loop))
                        0))
                    (crux:vector-len str)))
              (prelude:raise str 'prelude:string-position "not a string"))
         (prelude:raise ch 'prelude:string-position "not a char"))))

;;;
;;; string-find
;;;
(crux:intern prelude "string-find"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (crux:vector-len str))
                   ()
                   ((:lambda (len)
                       (crux:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (crux:less-than (crux:difference len 1) loop)
                                     ()
                                     (:if (crux:eq ch (crux:vector-ref str loop))
                                          ch
                                          (crux:sum 1 loop)))
                                loop))
                        0))
                    (crux:vector-len str)))
              (prelude:raise str 'prelude:string-find "not a string"))
         (prelude:raise ch 'prelude:string-find "not a char"))))

;;;
;;; string construction
;;;
(crux:intern prelude "string"
   (:lambda (designator)
      (:if (prelude:stringp designator)
         designator
         (:if (crux:eq :symbol (crux:type-of designator))
              (crux:symbol-name designator)
              (:if (prelude:charp designator)
                   (crux:make-vector :char `(,designator))
                   (prelude:raise designator 'prelude:string "not a string designator"))))))

(crux:intern prelude "string-append"
   (:lambda (list)
      (:if (prelude:listp list)
           ((:lambda (stream)
               (crux:fix
                (:lambda (list)
                   (:if list
                        ((:lambda ()
                            ((:lambda (str)
                                (:if (prelude:stringp str)
                                     (crux:write str () stream)
                                     (prelude:raise str 'prelude:string-append "is not a string")))
                             (crux:car list))
                            (crux:cdr list)))
                        ()))
                list)
               (crux:get-string stream))
            (crux:open :string :output ""))
           (prelude:raise list 'prelude:string "is not a list"))))

(crux:intern prelude "substr"
  (:lambda (str start end)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump start)
               (:if (prelude:fixnump end)
                    ((:lambda (substr)
                        (crux:fix
                         (:lambda (nth)
                            (:if (prelude:numberp nth)
                                 (:if (crux:eq nth (crux:vector-len str))
                                      ()
                                      (:if (crux:less-than nth (crux:sum 1 end))
                                           ((:lambda ()
                                               (crux:write (crux:vector-ref str nth) () substr)
                                               (crux:sum 1 nth)))
                                           ()))
                                 nth))
                         start)
                        (crux:get-string substr))
                     (crux:open :string :output ""))
                    (prelude:raise end 'prelude:substr "end is not a fixnum"))
               (prelude:raise str 'prelude:substr "is not a string"))
          (prelude:raise start 'prelude:substr "start is not a fixnum"))))

#|
(crux:intern prelude "substr-"
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
(crux:intern prelude "read-line"
   (:lambda (stream eof-error eof-value)
      (:if (prelude:streamp stream)
           ((:lambda (sstream)
               (crux:fix
                (:lambda (loop)
                   (:if (prelude:stringp loop)
                        loop
                        ((:lambda (ch)
                            (:if (crux:eq ch #\linefeed)
                                 (crux:get-string sstream)
                                 ((:lambda ()
                                     (crux:write-char ch sstream)
                                     (prelude:null loop)))))
                         (prelude:read-char stream eof-error eof-value))))
                ()))
            (crux:open :string :output ""))
           (prelude:raise str 'prelude:read-line "is not a stream"))))

;;;
;;; read from string
;;;
(crux:intern prelude "%read-string%" (crux:open :string :bidir ""))
(crux:intern prelude "read-string"
   (:lambda (str eof-error eof-value)
      (:if (prelude:stringp str)
           ((:lambda ()
              (crux:write str () prelude:%read-string%) 
              (crux:read prelude:%read-string% eof-error eof-value)))
           (prelude:raise str 'prelude:read-string "is not a string"))))

;;;
;;; write string
;;;
(crux:intern prelude "write-string"
   (:lambda (str designator)
      (:if (prelude:stringp str)
           ((:lambda (stream)
              (crux:write str () stream))
            (prelude:%read-stream-designator designator))
           (prelude:raise str 'prelude:write-string "is not a string"))))

;;;
;;; write line
;;;
(crux:intern prelude "write-line"
   (:lambda (str designator)
      ((:lambda (stream)          
         (prelude:write-string str stream)
         (prelude:write-char #\linefeed stream))
       (prelude:%write-stream-designator designator))))
