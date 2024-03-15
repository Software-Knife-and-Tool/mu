;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; armv8 aarch64 instruction
;;;
(mu:intern :arm "parser"
   (mu:open :string :bidir ""))

(mu:intern :arm "parse"
   (:lambda (line)
      (:if (prelude:zerop (mu:sv-len line))
           ()
           ((:lambda ()
               (mu:wr-char #\( arm:parser)
               (mu:fix
                  (:lambda (index)
                    (:if (mu:fx-lt index (mu:sv-len line))
                         ((:lambda (char)
                            (:if (prelude:%orf (mu:eq #\, char) (mu:eq #\# char))
                                 (prelude:1+ index)
                                 (:if (mu:eq #\/ char)
                                      (mu:sv-len line)
                                      ((:lambda ()
                                         (mu:wr-char char arm:parser)
                                         (prelude:1+ index))))))
                          (mu:sv-ref line index))
                         index))
                  0)
               (mu:wr-char #\) arm:parser))))
     (mu:read arm:parser () ())))

(mu:intern :arm "assemble-file"
   (:lambda (stream)
     (mu:fix
      (:lambda (line-number)
        ((:lambda (line)
           (:if (prelude:null line)
                line-number
                ((:lambda ()
                   (:if (mu:fx-lt line-number 10)
                        (prelude:format :t " ~A: ~A~%" `(,line-number ,line))
                        (prelude:format :t "~A: ~A~%" `(,line-number ,line)))
                   ((:lambda (op)
                      (:if (prelude:null op)
                           ()
                           (prelude:format :t "    #x~X~%" `(,op))))
                      (arm:armv8-assemble (arm:parse line)))
                   (prelude:1+ line-number)))))
         (prelude:read-line stream () ())))
      1)
     :t))

(mu:intern :arm "as"
   (:lambda (path)
      (:if (prelude:stringp path)
           (mu:with-ex
               (:lambda (exception)
                 (prelude:%exceptf mu:err-out () () exception))
               (:lambda ()
                 (arm:assemble-file (mu:open :file :input path))))
           (prelude:error "arm:as: path ~A is not a string" `(,path)))))
