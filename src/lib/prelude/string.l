;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(mu:intern prelude "stringp"
   (:lambda (string)
      (:if (mu:eq (mu:type-of string) :vector)
           (mu:eq :char (mu:vector-type string))
           (:if (prelude:%prelude-type-p string)
                (:if (mu:eq "vector" (mu:svref (mu:struct-vec string) 0))
                     (mu:eq :char (mu:vector-type (prelude:%vector-prop :base string)))
                     ())
                ()))))

(mu:intern prelude "get-output-stream-string"
   (:lambda (stream)
      (mu:get-string stream)))

(mu:intern prelude "schar"
  (:lambda (str index)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump index)
               (mu:svref str index)
               (prelude:raise index 'prelude:schar "not a string"))
          (prelude:raise str 'prelude:schar "not a string"))))

(mu:intern prelude "string="
   (:lambda (str-1 str-2)
      (:if (prelude:stringp str-1)
           (:if (prelude:stringp str-2)
                (:if (mu:eq str-1 str-2)
                     :t
                     (:if (mu:eq (mu:vector-len str-1) (mu:vector-len str-2))
                          (mu:fix
                           (:lambda (nth)
                              (:if (prelude:numberp nth)
                                   (:if (mu:eq nth (mu:vector-len str-1))
                                        :t
                                        (:if (mu:eq (mu:svref str-1 nth) (mu:svref str-2 nth))
                                             (mu:sum 1 nth)
                                             ()))
                                   nth))
                           0)
                          ()))
                (prelude:raise str-2 'prelude:string= "not a string"))
           (prelude:raise str-1 'prelude:string= "not a string"))))

(mu:intern prelude "%string-write"
   (:lambda (string escape stream)
      (:if (mu:eq :vector (mu:type-of string))
           (mu:write string escape stream)
           ((:lambda (length)
               (:if escape
                    (mu:write #\" () stream)
                    ())
               (mu:fix
                (:lambda (index)
                   (:if (mu:less-than index length)
                        ((:lambda ()
                            (prelude:write (prelude:vector-ref string index) () stream)
                            (prelude:1+ index)))
                        index))
                0)
               (:if escape
                    (mu:write #\" () stream)
                    ()))
            (prelude:%vector-prop :length string)))
      ()))

;;;
;;; string-position
;;;
(mu:intern prelude "string-position"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (mu:vector-len str))
                   ()
                   ((:lambda (len)
                       (mu:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (mu:less-than (mu:difference len 1) loop)
                                     ()
                                     (:if (mu:eq ch (mu:svref str loop))
                                          loop
                                          (mu:sum 1 loop)))
                                loop))
                        0))
                    (mu:vector-len str)))
              (prelude:raise str 'prelude:string-position "not a string"))
         (prelude:raise ch 'prelude:string-position "not a char"))))

;;;
;;; string-find
;;;
(mu:intern prelude "string-find"
  (:lambda (ch str)
    (:if (prelude:charp ch)
         (:if (prelude:stringp str)
              (:if (prelude:zerop (mu:vector-len str))
                   ()
                   ((:lambda (len)
                       (mu:fix
                        (:lambda (loop)
                           (:if (prelude:numberp loop)
                                (:if (mu:less-than (mu:difference len 1) loop)
                                     ()
                                     (:if (mu:eq ch (mu:svref str loop))
                                          ch
                                          (mu:sum 1 loop)))
                                loop))
                        0))
                    (mu:vector-len str)))
              (prelude:raise str 'prelude:string-find "not a string"))
         (prelude:raise ch 'prelude:string-find "not a char"))))

;;;
;;; string construction
;;;
(mu:intern prelude "string"
   (:lambda (designator)
      (:if (prelude:stringp designator)
         designator
         (:if (mu:eq :symbol (mu:type-of designator))
              (mu:symbol-name designator)
              (:if (prelude:charp designator)
                   (mu:make-vector :char `(,designator))
                   (prelude:raise designator 'prelude:string "not a string designator"))))))

(mu:intern prelude "string-append"
   (:lambda (list)
      (:if (prelude:listp list)
           ((:lambda (stream)
               (mu:fix
                (:lambda (list)
                   (:if list
                        ((:lambda ()
                            ((:lambda (str)
                                (:if (prelude:stringp str)
                                     (mu:write str () stream)
                                     (prelude:raise str 'prelude:string-append "is not a string")))
                             (mu:car list))
                            (mu:cdr list)))
                        ()))
                list)
               (mu:get-string stream))
            (mu:open :string :output ""))
           (prelude:raise list 'prelude:string "is not a list"))))

(mu:intern prelude "substr"
  (:lambda (str start end)
     (:if (prelude:stringp str)
          (:if (prelude:fixnump start)
               (:if (prelude:fixnump end)
                    ((:lambda (substr)
                        (mu:fix
                         (:lambda (nth)
                            (:if (prelude:numberp nth)
                                 (:if (mu:eq nth (mu:vector-len str))
                                      ()
                                      (:if (mu:less-than nth (mu:sum 1 end))
                                           ((:lambda ()
                                               (mu:write (mu:svref str nth) () substr)
                                               (mu:sum 1 nth)))
                                           ()))
                                 nth))
                         start)
                        (mu:get-string substr))
                     (mu:open :string :output ""))
                    (prelude:raise end 'prelude:substr "end is not a fixnum"))
               (prelude:raise str 'prelude:substr "is not a string"))
          (prelude:raise start 'prelude:substr "start is not a fixnum"))))

#|
(mu:intern prelude "substr-"
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
(mu:intern prelude "read-line"
   (:lambda (stream eof-error eof-value)
      (:if (prelude:streamp stream)
           ((:lambda (sstream)
               (mu:fix
                (:lambda (loop)
                   (:if (prelude:stringp loop)
                        loop
                        ((:lambda (ch)
                            (:if (mu:eq ch #\linefeed)
                                 (mu:get-string sstream)
                                 ((:lambda ()
                                     (mu:write-char ch sstream)
                                     (prelude:null loop)))))
                         (prelude:read-char stream eof-error eof-value))))
                ()))
            (mu:open :string :output ""))
           (prelude:raise str 'prelude:read-line "is not a stream"))))

;;;
;;; read from string
;;;
(mu:intern prelude "%read-string%" (mu:open :string :bidir ""))
(mu:intern prelude "read-string"
   (:lambda (str eof-error eof-value)
      (:if (prelude:stringp str)
           ((:lambda ()
              (mu:write str () prelude:%read-string%) 
              (mu:read prelude:%read-string% eof-error eof-value)))
           (prelude:raise str 'prelude:read-string "is not a string"))))

;;;
;;; write string
;;;
(mu:intern prelude "write-string"
   (:lambda (str designator)
      (:if (prelude:stringp str)
           ((:lambda (stream)
              (mu:write str () stream))
            (prelude:%read-stream-designator designator))
           (prelude:raise str 'prelude:write-string "is not a string"))))

;;;
;;; write line
;;;
(mu:intern prelude "write-line"
   (:lambda (str designator)
      ((:lambda (stream)          
         (prelude:write-string str stream)
         (prelude:write-char #\linefeed stream))
       (prelude:%write-stream-designator designator))))
