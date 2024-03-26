;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; armv8 aarch64 instruction
;;;
(libcore:intern :arm "parser"
   (libcore:open :string :bidir ""))

(libcore:intern :arm "parse"
   (:lambda (line)
      (:if (prelude:zerop (libcore:sv-len line))
           ()
           ((:lambda ()
               (libcore:wr-char #\( arm:parser)
               (libcore:fix
                  (:lambda (index)
                    (:if (libcore:fx-lt index (libcore:sv-len line))
                         ((:lambda (char)
                            (:if (prelude:%orf (libcore:eq #\, char) (libcore:eq #\# char))
                                 (prelude:1+ index)
                                 (:if (libcore:eq #\/ char)
                                      (libcore:sv-len line)
                                      ((:lambda ()
                                         (libcore:wr-char char arm:parser)
                                         (prelude:1+ index))))))
                          (libcore:sv-ref line index))
                         index))
                  0)
               (libcore:wr-char #\) arm:parser))))
     (libcore:read arm:parser () ())))

(libcore:intern :arm "assemble-file"
   (:lambda (stream)
     (libcore:fix
      (:lambda (line-number)
        ((:lambda (line)
           (:if (prelude:null line)
                line-number
                ((:lambda ()
                   (:if (libcore:fx-lt line-number 10)
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

(libcore:intern :arm "as"
   (:lambda (path)
      (:if (prelude:stringp path)
           (libcore:with-ex
               (:lambda (exception)
                 (prelude:%exceptf libcore:err-out () () exception))
               (:lambda ()
                 (arm:assemble-file (libcore:open :file :input path))))
           (prelude:error "arm:as: path ~A is not a string" `(,path)))))
