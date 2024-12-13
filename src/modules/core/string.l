;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; strings
;;;
(mu:intern core "stringp"
   (:lambda (string)
      (:if (mu:eq (mu:type-of string) :vector)
           (mu:eq :char (mu:vector-type string))
           (:if (core:%core-type-p string)
                (:if (mu:eq "vector" (mu:svref (mu:struct-vec string) 0))
                     (mu:eq :char (mu:vector-type (core:%vector-prop :base string)))
                     ())
                ()))))

(mu:intern core "%string-position"
  (:lambda (ch str)
    (:if (mu:eq 0 (mu:vector-length str))
         ()
         ((:lambda (len)
            (mu:fix
             (:lambda (loop)
               (:if (core:numberp loop)
                    (:if (mu:less-than (mu:sub len 1) loop)
                         ()
                         (:if (mu:eq ch (mu:svref str loop))
                              loop
                              (mu:add 1 loop)))
                    loop))
             0))
          (mu:vector-length str)))))

(mu:intern core "%substr"
  (:lambda (str start end)
    ((:lambda (substr)
       (mu:fix
        (:lambda (nth)
          (:if (core:numberp nth)
               (:if (mu:eq nth (mu:vector-length str))
                    ()
                    (:if (mu:less-than nth (mu:add 1 end))
                         (core:%prog2
                            (mu:write (mu:svref str nth) () substr)
                            (mu:add 1 nth))
                         ()))
               nth))
        start)
       (mu:get-string substr))
     (mu:open :string :output "" :t))))
