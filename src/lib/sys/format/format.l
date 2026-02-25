;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; format sys module
;;;
(mu:intern format "%fmt-controls"
   `(,(mu:cons #\A
               (:lambda (dest arg-list)
                 (mu:write (mu:car arg-list) () dest)
                 (mu:cdr arg-list)))
      ,(mu:cons #\S
                (:lambda (dest arg-list)
                  (mu:write (mu:car arg-list) :t dest)
                  (mu:cdr arg-list)))
      ,(mu:cons #\X
                (:lambda (dest arg-list)
                  ((:lambda (fx)
                     (:if (mu:eq :fixnum (mu:type-of fx))
                          (:if (mu:eq 0 fx)
                               (mu:write "00" () dest)
                               ((:lambda (str-stream)
                                  (mu:fix
                                   (:lambda (n)
                                     (:if (:if (mu:eq () n)
                                           :t
                                           (mu:eq 0 n))
                                          ()
                                          ((:lambda ()
                                           (mu:write (mu:svref "0123456789abcdef" (mu:logand n #xf)) () str-stream)
                                           (mu:div n 16)))))
                                   (:if (mu:less-than fx 0) (mu:mul -1 fx) fx))
                                  (:if (mu:less-than fx 0)
                                       (mu:write #\- () str-stream)
                                       ())
                                  ((:lambda (hexstr)
                                     (mu:fix
                                      (:lambda (len)
                                        (:if (mu:eq 0 len)
                                             0
                                             ((:lambda ()
                                              (mu:write (mu:svref hexstr (mu:sub len 1)) () dest)
                                              (mu:sub len 1)))))
                                      (mu:vector-length hexstr)))
                                   (mu:get-string str-stream)))
                                (mu:open :string :output "" :t)))
                          (mu:raise fx 'format:%format :type)))
                   (mu:car arg-list))))
      ,(mu:cons #\D
                (:lambda (dest arg-list)
                  (:if (mu:eq :fixnum (mu:type-of (mu:car arg-list)))
                       ((:lambda ()
                        (mu:write (mu:car arg-list) () dest)
                        (mu:cdr arg-list)))
                       (mu:raise (mu:car arg-list) 'format:%format :type))))
      ,(mu:cons #\T
                (:lambda (dest arg-list)
                  (mu:write-char #\tab dest)
                  arg-list))
      ,(mu:cons #\~
                (:lambda (dest arg-list)
                  (mu:write-char #\~ dest)
                  arg-list))
      ,(mu:cons #\%
                (:lambda (dest arg-list)
                  (mu:write-char #\linefeed dest)
                  arg-list))
      ,(mu:cons #\!
                (:lambda (dest arg-list)
                  (mu:cdr arg-list)))))

(mu:intern format "%fmt-op"
  (:lambda (dir dest arg-list)
    ((:lambda (fn)
       (:if fn
            (mu:apply fn (mu:cons dest (mu:cons arg-list ())))
            (mu:raise dir 'format:%format :range)))
     (mu:cdr
      (mu:car
       (mu:fix
        (:lambda (list)
           (:if list
                ((:lambda (entry cdr)
                    (:if entry
                         (:if (mu:eq dir (mu:car entry))
                              list
                              cdr)
                         cdr))
                 (mu:car list)
                 (mu:cdr list))
                ()))
        format:%fmt-controls))))))

(mu:intern format "%fmt-loop"
  (:lambda (stream dest fmt arg-list)
    (mu:fix
     (:lambda (loop)
       ((:lambda (nth argl)
          (:if (mu:eq nth (mu:vector-length fmt))
               loop
               ((:lambda (ch)
                  (:if (mu:eq ch #\~)
                       (mu:cons (mu:add 2 nth) (format:%fmt-op (mu:svref fmt (mu:add 1 nth)) dest argl))
                       ((:lambda ()
                           (mu:write-char ch dest)
                           (mu:cons (mu:add 1 nth) argl)))))
                (mu:svref fmt nth))))
        (mu:car loop)
        (mu:cdr loop)))
     (mu:cons 0 arg-list))
    (:if (mu:eq () stream)
         (mu:get-string dest)
         ())))

(mu:intern format "%format"
   (:lambda (stream format-string arg-list)
     (:if (:if (mu:eq :vector (mu:type-of format-string))
               (mu:eq :char (mu:vector-type format-string))
               ())
          (:if (:if (mu:eq () arg-list)
                    :t
                    (mu:eq :cons (mu:type-of arg-list)))
               (:if (mu:eq 0 (mu:vector-length format-string))
                    (:if stream
                         ""
                         ())
                    ((:lambda (dest) (format:%fmt-loop stream dest format-string arg-list))
                     (:if (mu:eq () stream)
                          (mu:open :string :output "" :t)
                          (:if (mu:eq stream :t)
                               mu:*standard-output*
                               (:if (mu:eq :stream (mu:type-of stream))
                                    stream
                                    (mu:raise stream 'format:%format :type))))))
               (mu:raise arg-list 'format:%format :type))
          (mu:raise format-string 'format:%format :type))))
