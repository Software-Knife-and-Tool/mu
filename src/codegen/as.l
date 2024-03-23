;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; armv8 aarch64 instruction
;;;
(core:intern :arm "parser"
   (core:open :string :bidir ""))

(core:intern :arm "parse"
   (:lambda (line)
      (:if (prelude:zerop (core:sv-len line))
           ()
           ((:lambda ()
               (core:wr-char #\( arm:parser)
               (core:fix
                  (:lambda (index)
                    (:if (core:fx-lt index (core:sv-len line))
                         ((:lambda (char)
                            (:if (prelude:%orf (core:eq #\, char) (core:eq #\# char))
                                 (prelude:1+ index)
                                 (:if (core:eq #\/ char)
                                      (core:sv-len line)
                                      ((:lambda ()
                                         (core:wr-char char arm:parser)
                                         (prelude:1+ index))))))
                          (core:sv-ref line index))
                         index))
                  0)
               (core:wr-char #\) arm:parser))))
     (core:read arm:parser () ())))

(core:intern :arm "assemble-file"
   (:lambda (stream)
     (core:fix
      (:lambda (line-number)
        ((:lambda (line)
           (:if (prelude:null line)
                line-number
                ((:lambda ()
                   (:if (core:fx-lt line-number 10)
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

(core:intern :arm "as"
   (:lambda (path)
      (:if (prelude:stringp path)
           (core:with-ex
               (:lambda (exception)
                 (prelude:%exceptf core:err-out () () exception))
               (:lambda ()
                 (arm:assemble-file (core:open :file :input path))))
           (prelude:error "arm:as: path ~A is not a string" `(,path)))))
