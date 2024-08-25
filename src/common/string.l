;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(mu:intern common "stringp"
   (:lambda (string)
      (:if (mu:eq (mu:type-of string) :vector)
           (mu:eq :char (mu:vector-type string))
           (:if (common:%common-type-p string)
                (:if (mu:eq "vector" (mu:svref (mu:struct-vec string) 0))
                     (mu:eq :char (mu:vector-type (common:%vector-prop :base string)))
                     ())
                ()))))

(mu:intern common "get-output-stream-string"
   (:lambda (stream)
      (mu:get-string stream)))

(mu:intern common "schar"
  (:lambda (str index)
     (:if (common:stringp str)
          (:if (common:fixnump index)
               (mu:svref str index)
               (common:raise index 'common:schar "not a string"))
          (common:raise str 'common:schar "not a string"))))

(mu:intern common "string="
   (:lambda (str-1 str-2)
      (:if (common:stringp str-1)
           (:if (common:stringp str-2)
                (:if (mu:eq str-1 str-2)
                     :t
                     (:if (mu:eq (mu:vector-len str-1) (mu:vector-len str-2))
                          (mu:fix
                           (:lambda (nth)
                              (:if (common:numberp nth)
                                   (:if (mu:eq nth (mu:vector-len str-1))
                                        :t
                                        (:if (mu:eq (mu:svref str-1 nth) (mu:svref str-2 nth))
                                             (mu:sum 1 nth)
                                             ()))
                                   nth))
                           0)
                          ()))
                (common:raise str-2 'common:string= "not a string"))
           (common:raise str-1 'common:string= "not a string"))))

(mu:intern common "%string-write"
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
                            (common:write (common:vector-ref string index) () stream)
                            (common:1+ index)))
                        index))
                0)
               (:if escape
                    (mu:write #\" () stream)
                    ()))
            (common:%vector-prop :length string)))
      ()))

(mu:intern common "string-position"
  (:lambda (ch str)
    (:if (common:charp ch)
         (:if (common:stringp str)
              (:if (common:zerop (mu:vector-len str))
                   ()
                   ((:lambda (len)
                       (mu:fix
                        (:lambda (loop)
                           (:if (common:numberp loop)
                                (:if (mu:less-than (mu:difference len 1) loop)
                                     ()
                                     (:if (mu:eq ch (mu:svref str loop))
                                          loop
                                          (mu:sum 1 loop)))
                                loop))
                        0))
                    (mu:vector-len str)))
              (common:raise str 'common:string-position "not a string"))
         (common:raise ch 'common:string-position "not a char"))))

(mu:intern common "string-find"
  (:lambda (ch str)
    (:if (common:charp ch)
         (:if (common:stringp str)
              (:if (common:zerop (mu:vector-len str))
                   ()
                   ((:lambda (len)
                       (mu:fix
                        (:lambda (loop)
                           (:if (common:numberp loop)
                                (:if (mu:less-than (mu:difference len 1) loop)
                                     ()
                                     (:if (mu:eq ch (mu:svref str loop))
                                          ch
                                          (mu:sum 1 loop)))
                                loop))
                        0))
                    (mu:vector-len str)))
              (common:raise str 'common:string-find "not a string"))
         (common:raise ch 'common:string-find "not a char"))))

(mu:intern common "string"
   (:lambda (designator)
      (:if (common:stringp designator)
         designator
         (:if (mu:eq :symbol (mu:type-of designator))
              (mu:symbol-name designator)
              (:if (common:charp designator)
                   (mu:make-vector :char `(,designator))
                   (common:raise designator 'common:string "not a string designator"))))))

(mu:intern common "string-append"
   (:lambda (list)
      (:if (common:listp list)
           ((:lambda (stream)
               (mu:fix
                (:lambda (list)
                   (:if list
                        ((:lambda ()
                            ((:lambda (str)
                                (:if (common:stringp str)
                                     (mu:write str () stream)
                                     (common:raise str 'common:string-append "is not a string")))
                             (mu:car list))
                            (mu:cdr list)))
                        ()))
                list)
               (mu:get-string stream))
            (mu:open :string :output ""))
           (common:raise list 'common:string "is not a list"))))

(mu:intern common "substr"
  (:lambda (str start end)
     (:if (common:stringp str)
          (:if (common:fixnump start)
               (:if (common:fixnump end)
                    ((:lambda (substr)
                        (mu:fix
                         (:lambda (nth)
                            (:if (common:numberp nth)
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
                    (common:raise end 'common:substr "end is not a fixnum"))
               (common:raise str 'common:substr "is not a string"))
          (common:raise start 'common:substr "start is not a fixnum"))))

#|
(mu:intern common "substr-"
  (:lambda (str start end)
     (:if (common:stringp str)
          (:if (common:fixnump start)
               (:if (common:fixnump end)
                    (common:vector-slice str start end)
                    (common:raise end 'common:substr "is not a fixnum"))
               (common:raise str 'common:substr "is not a string"))
          (common:raise start 'common:substr "is not a fixnum"))))

(mu:intern common "read-line"
   (:lambda (stream eof-error eof-value)
      (:if (common:streamp stream)
           ((:lambda (sstream)
               (mu:fix
                (:lambda (loop)
                   (:if (common:stringp loop)
                        loop
                        ((:lambda (ch)
                            (:if (mu:eq ch #\linefeed)
                                 (mu:get-string sstream)
                                 ((:lambda ()
                                     (mu:write-char ch sstream)
                                     (common:null loop)))))
                         (common:read-char stream eof-error eof-value))))
                ()))
            (mu:open :string :output ""))
           (common:raise str 'common:read-line "is not a stream"))))
|#

(mu:intern common "%read-string%" (mu:open :string :bidir ""))
(mu:intern common "read-string"
   (:lambda (str eof-error eof-value)
      (:if (common:stringp str)
           ((:lambda ()
              (mu:write str () common:%read-string%) 
              (mu:read common:%read-string% eof-error eof-value)))
           (common:raise str 'common:read-string "is not a string"))))

(mu:intern common "write-string"
   (:lambda (str designator)
      (:if (common:stringp str)
           ((:lambda (stream)
              (mu:write str () stream))
            (common:read-stream-designator designator))
           (common:raise str 'common:write-string "is not a string"))))

(mu:intern common "write-line"
   (:lambda (str designator)
      ((:lambda (stream)          
         (common:write-string str stream)
         (common:write-char #\linefeed stream))
       (common:write-stream-designator designator))))
