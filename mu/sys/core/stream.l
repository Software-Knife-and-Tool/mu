;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;

;;;
;;; load
;;;
(mu:intern core "load"
   (:lambda (path)
     (:if (core:stringp path)
          ((:lambda (stream)
             (mu:fix
              (:lambda (loop)
                (:if (mu:eq loop core:%eof%)
                     core:%eof%
                     ((:lambda (form)
                        (:if (mu:eq form core:%eof%)
                             core:%eof%
                             (core:%prog2
                                (mu:eval (core:compile form))
                                (core:null loop))))
                      (core:read stream () core:%eof%))))
              ()))
           (mu:open :file :input path :t))
          (mu:raise-from path 'core:load :type))
     :t))

;;;
;;; format
;;;
(mu:intern core "%fmt-controls"
   `(,(mu:cons #\A (:lambda (dest arg-list)
                     (core:write (mu:car arg-list) () dest)
                     (mu:cdr arg-list)))
      ,(mu:cons #\S (:lambda (dest arg-list)
                      (core:write (mu:car arg-list) :t dest)
                      (mu:cdr arg-list)))
      ,(mu:cons #\X (:lambda (dest arg-list)
                      ((:lambda (fx)
                         (:if (core:fixnump fx)
                              (:if (mu:eq 0 fx)
                                   (core:write "00" () dest)
                                   ((:lambda (str-stream)
                                      (mu:fix
                                       (:lambda (n)
                                         (:if (core:%or (core:null n) (mu:eq 0 n))
                                              ()
                                              (core:%prog2
                                               (mu:write (mu:svref "0123456789abcdef" (mu:logand n #xf)) () str-stream)
                                               (mu:div n 16))))
                                       (:if (mu:less-than fx 0) (mu:mul -1 fx) fx))
                                      (:if (mu:less-than fx 0)
                                           (mu:write #\- () str-stream)
                                           ())
                                      ((:lambda (hexstr)
                                         (mu:fix
                                          (:lambda (len)
                                            (:if (mu:eq 0 len)
                                                 0
                                                 (core:%prog2
                                                  (mu:write (mu:svref hexstr (mu:sub len 1)) () dest)
                                                  (mu:sub len 1))))
                                          (mu:vector-length hexstr)))
                                       (mu:get-string str-stream)))
                                    (mu:open :string :output "" :t)))
                              (mu:raise-from fx 'core:%format :type)))
                       (mu:car arg-list))))
      ,(mu:cons #\D   (:lambda (dest arg-list)
                        (:if (core:fixnump (mu:car arg-list))
                             (core:%prog2
                              (core:write (mu:car arg-list) () dest)
                              (mu:cdr arg-list))
                             (mu:raise-from (mu:car arg-list) 'core:%format :type))))
      ,(mu:cons #\T (:lambda (dest arg-list)
                      (mu:write-char #\tab dest)
                      arg-list))
      ,(mu:cons #\~ (:lambda (dest arg-list)
                      (mu:write-char #\~ dest)
                      arg-list))
      ,(mu:cons #\% (:lambda (dest arg-list)
                      (mu:write-char #\linefeed dest)
                      arg-list))
      ,(mu:cons #\! (:lambda (dest arg-list)
                      (mu:cdr arg-list)))))

(mu:intern core "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (core:apply fn (core:%list2 dest arg-list))
            (mu:raise-from dir 'core:%format :range)))
     (mu:cdr (core:assq dir core:%fmt-controls)))))

(mu:intern core "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (mu:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (mu:eq nth (mu:vector-length fmt))
               loop
               ((:lambda (ch)
                  (:if (mu:eq ch #\~)
                       (mu:cons (mu:add 2 nth) (core:%fmt-op (mu:svref fmt (mu:add 1 nth)) dest argl))
                       (core:%prog2
                           (mu:write-char ch dest)
                           (mu:cons (mu:add 1 nth) argl))))
                (mu:svref fmt nth))))
        (mu:car loop)
        (mu:cdr loop)))
     (mu:cons 0 arg-list))
    (:if (core:null stream)
         (mu:get-string dest)
         ())))

(mu:intern core "%format"
   (:lambda (stream format-string arg-list)
      (:if (core:stringp format-string)
           (:if (core:listp arg-list)
                (:if (mu:eq 0 (mu:vector-length format-string))
                     (:if stream
                          ""
                          ())
                     ((:lambda (dest) (core:%fmt-loop stream dest format-string arg-list))
                      (:if (core:null stream)
                           (mu:open :string :output "" :t)
                           (:if (mu:eq stream :t)
                                mu:*standard-output*
                                (:if (core:streamp stream)
                                     stream
                                     (mu:raise-from stream 'core:%format :type))))))
                (mu:raise-from arg-list 'core:%format :type))
           (mu:raise-from format-string 'core:%format :type))))

;;;
;;; functions
;;;
(mu:intern core "%peek-char"
   (:lambda (stream error-eofp eof-value)
    (mu:unread-char
     (mu:read-char stream error-eofp eof-value)
     stream)))

;;;
;;; read/write
;;;
(mu:intern core "read"
   (:lambda (stream eof-error eof-value)
     (:if (core:streamp stream)
          ((:lambda (form)
             (:if (mu:eq form core:%read-list-eol%)
                  (mu:raise-from stream 'core:read :syntax)
                  (:if (mu:eq core:%eof% form)
                       (:if eof-error
                            (mu:raise-from stream 'core:read :eof)
                            eof-value)
                       form)))
           (core:%read stream))
          (mu:raise-from stream 'core:read :type))))

(mu:intern core "write"
   (:lambda (form escape stream)
     (:if (core:streamp stream)
          (:if (core:%and (core:vectorp form) (core:%typep form))
               (core:%vector-write form escape stream)
               (:if (mu:eq :symbol (mu:type-of form))
                    (:if (core:null escape)
                         ((:lambda (ns name)
                            (:if (mu:eq core:%reader/ ns)
                                 (mu:write name () stream)
                                 (mu:write form () stream)))
                            (mu:symbol-namespace form)
                            (mu:symbol-name form))
                         (mu:write form escape stream))
               (mu:write form escape stream)))
          (mu:raise-from stream 'core:write :type))))
