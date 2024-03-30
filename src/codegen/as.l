;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; armv8 aarch64 instruction
;;;
(lib:intern :arm "parser"
   (lib:open :string :bidir ""))

(lib:intern :arm "parse"
   (:lambda (line)
      (:if (prelude:zerop (lib:sv-len line))
           ()
           ((:lambda ()
               (lib:wr-char #\( arm:parser)
               (lib:fix
                  (:lambda (index)
                    (:if (lib:fx-lt index (lib:sv-len line))
                         ((:lambda (char)
                            (:if (prelude:%orf (lib:eq #\, char) (lib:eq #\# char))
                                 (prelude:1+ index)
                                 (:if (lib:eq #\/ char)
                                      (lib:sv-len line)
                                      ((:lambda ()
                                         (lib:wr-char char arm:parser)
                                         (prelude:1+ index))))))
                          (lib:sv-ref line index))
                         index))
                  0)
               (lib:wr-char #\) arm:parser))))
     (lib:read arm:parser () ())))

(lib:intern :arm "assemble-file"
   (:lambda (stream)
     (lib:fix
      (:lambda (line-number)
        ((:lambda (line)
           (:if (prelude:null line)
                line-number
                ((:lambda ()
                   (:if (lib:fx-lt line-number 10)
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

(lib:intern :arm "as"
   (:lambda (path)
      (:if (prelude:stringp path)
           (lib:with-ex
               (:lambda (exception)
                 (prelude:%exceptf lib:err-out () () exception))
               (:lambda ()
                 (arm:assemble-file (lib:open :file :input path))))
           (prelude:error "arm:as: path ~A is not a string" `(,path)))))
