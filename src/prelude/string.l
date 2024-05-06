;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(lib:intern prelude "stringp"
   (:lambda (string)
      (:if (lib:eq (lib:type-of string) :vector)
           (lib:eq :char (lib:vector-type string))
           (:if (prelude:%prelude-type-p string)
                (:if (lib:eq "vector" (lib:vector-ref (lib:struct-vec string) 0))
                     (lib:eq :char (lib:vector-type (prelude:%vector-prop :base string)))
                     ())
                ()))))

(lib:intern prelude "get-output-stream-string"
   (:lambda (stream)
      (lib:get-string stream)))

(lib:intern prelude "schar"
  (:lambda (str index)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump index)
               (lib:vector-ref str index)
               (prelude:raise index 'prelude:schar "not a string"))
          (prelude:raise str 'prelude:schar "not a string"))))

(lib:intern prelude "string="
   (:lambda (str-1 str-2)
      (:if (prelude:stringp str-1)
           (:if (prelude:stringp str-2)
                (:if (lib:eq str-1 str-2)
                     :t
                     (:if (lib:eq (lib:vector-len str-1) (lib:vector-len str-2))
                          (lib:fix
                           (:lambda (nth)
                              (:if (prelude:numberp nth)
                                   (:if (lib:eq nth (lib:vector-len str-1))
                                        :t
                                        (:if (lib:eq (lib:vector-ref str-1 nth) (lib:vector-ref str-2 nth))
                                             (lib:fx-add 1 nth)
                                             ()))
                                   nth))
                           0)
                          ()))
                (prelude:raise str-2 'prelude:string= "not a string"))
           (prelude:raise str-1 'prelude:string= "not a string"))))

(lib:intern prelude "%string-write"
   (:lambda (string escape stream)
      (:if (lib:eq :vector (lib:type-of string))
           (lib:write string escape stream)
           ((:lambda (length)
               (:if escape
                    (lib:write #\" () stream)
                    ())
               (lib:fix
                (:lambda (index)
                   (:if (lib:fx-lt index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref string index) () stream)
                            (prelude:1+ index)))
                        index))
                0)
               (:if escape
                    (lib:write #\" () stream)
                    ()))
            (prelude:%vector-prop :length string)))
      ()))

;;;
;;; string-position
;;;
(lib:intern prelude "string-position"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (lib:vector-len str))
                   ()
                   ((:lambda (len)
                       (lib:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (lib:fx-lt (lib:fx-sub len 1) loop)
                                     ()
                                     (:if (lib:eq ch (lib:vector-ref str loop))
                                          loop
                                          (lib:fx-add 1 loop)))
                                loop))
                        0))
                    (lib:vector-len str)))
              (prelude:raise str 'prelude:string-position "not a string"))
         (prelude:raise ch 'prelude:string-position "not a char"))))

;;;
;;; string-find
;;;
(lib:intern prelude "string-find"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (lib:vector-len str))
                   ()
                   ((:lambda (len)
                       (lib:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (lib:fx-lt (lib:fx-sub len 1) loop)
                                     ()
                                     (:if (lib:eq ch (lib:vector-ref str loop))
                                          ch
                                          (lib:fx-add 1 loop)))
                                loop))
                        0))
                    (lib:vector-len str)))
              (prelude:raise str 'prelude:string-find "not a string"))
         (prelude:raise ch 'prelude:string-find "not a char"))))

;;;
;;; string construction
;;;
(lib:intern prelude "string"
   (:lambda (designator)
      (:if (prelude:stringp designator)
         designator
         (:if (lib:eq :symbol (lib:type-of designator))
              (lib:symbol-name designator)
              (:if (prelude:charp designator)
                   (lib:make-vector :char `(,designator))
                   (prelude:raise designator 'prelude:string "not a string designator"))))))

(lib:intern prelude "string-append"
   (:lambda (list)
      (:if (prelude:listp list)
           ((:lambda (stream)
               (lib:fix
                (:lambda (list)
                   (:if list
                        ((:lambda ()
                            ((:lambda (str)
                                (:if (prelude:stringp str)
                                     (lib:write str () stream)
                                     (prelude:raise str 'prelude:string-append "is not a string")))
                             (lib:car list))
                            (lib:cdr list)))
                        ()))
                list)
               (lib:get-string stream))
            (lib:open :string :output ""))
           (prelude:raise list 'prelude:string "is not a list"))))

(lib:intern prelude "substr"
  (:lambda (str start end)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump start)
               (:if (prelude:fixnump end)
                    ((:lambda (substr)
                        (lib:fix
                         (:lambda (nth)
                            (:if (prelude:numberp nth)
                                 (:if (lib:eq nth (lib:vector-len str))
                                      ()
                                      (:if (lib:fx-lt nth (lib:fx-add 1 end))
                                           ((:lambda ()
                                               (lib:write (lib:vector-ref str nth) () substr)
                                               (lib:fx-add 1 nth)))
                                           ()))
                                 nth))
                         start)
                        (lib:get-string substr))
                     (lib:open :string :output ""))
                    (prelude:raise end 'prelude:substr "end is not a fixnum"))
               (prelude:raise str 'prelude:substr "is not a string"))
          (prelude:raise start 'prelude:substr "start is not a fixnum"))))

#|
(lib:intern prelude "substr-"
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
(lib:intern prelude "read-line"
   (:lambda (stream eof-error eof-value)
      (:if (prelude:streamp stream)
           ((:lambda (sstream)
               (lib:fix
                (:lambda (loop)
                   (:if (prelude:stringp loop)
                        loop
                        ((:lambda (ch)
                            (:if (lib:eq ch #\linefeed)
                                 (lib:get-string sstream)
                                 ((:lambda ()
                                     (lib:write-char ch sstream)
                                     (prelude:null loop)))))
                         (prelude:read-char stream eof-error eof-value))))
                ()))
            (lib:open :string :output ""))
           (prelude:raise str 'prelude:read-line "is not a stream"))))

;;;
;;; read from string
;;;
(lib:intern prelude "%read-string%" (lib:open :string :bidir ""))
(lib:intern prelude "read-string"
   (:lambda (str eof-error eof-value)
      (:if (prelude:stringp str)
           ((:lambda ()
              (lib:write str () prelude:%read-string%) 
              (lib:read prelude:%read-string% eof-error eof-value)))
           (prelude:raise str 'prelude:read-string "is not a string"))))

;;;
;;; write string
;;;
(lib:intern prelude "write-string"
   (:lambda (str designator)
      (:if (prelude:stringp str)
           ((:lambda (stream)
              (lib:write str () stream))
            (prelude:%read-stream-designator designator))
           (prelude:raise str 'prelude:write-string "is not a string"))))

;;;
;;; write line
;;;
(lib:intern prelude "write-line"
   (:lambda (str designator)
      ((:lambda (stream)          
         (prelude:write-string str stream)
         (prelude:write-char #\linefeed stream))
       (prelude:%write-stream-designator designator))))
