;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  room
;;;
(mu:intern common "%heap-delta-apply"
  (:lambda (fn args)
    ((:lambda (start-vec stop-vec)
       (mu:fix
        (:lambda (loop)
          (:if (mu:eq :vector (mu:type-of loop))
               loop
               ((:lambda (list n)
                  ((:lambda (offset)
                     (:if (mu:less-than offset (mu:vector-length start-vec))
                          ((:lambda (type total alloc in-use)
                             ((:lambda (type-stats)
                                (mu:cons `(,@list ,@type-stats) (mu:add 1 n)))
                              `(,type ,total ,alloc ,in-use)))
                           (mu:svref start-vec (mu:add offset 0))
                           (mu:sub (mu:svref stop-vec (mu:add offset 1)) (mu:svref start-vec (mu:add offset 1)))
                           (mu:sub (mu:svref stop-vec (mu:add offset 2)) (mu:svref start-vec (mu:add offset 2)))
                           (mu:sub (mu:svref stop-vec (mu:add offset 3)) (mu:svref start-vec (mu:add offset 3))))
                          (mu:make-vector :t list)))
                         (mu:mul n 4)))
                 (mu:car loop)
                 (mu:cdr loop))))
         (mu:cons () 1)))
     (mu:heap-stat)
     ((:lambda ()
        (mu:apply fn args)
        (mu:heap-stat))))))

(mu:intern common "room"
   (:lambda (verbosity)
     ((:lambda (level-key)
        (:if (core:%findl-if (:lambda (level) (mu:eq level level-key)) '(:t :null :default))
             ((:lambda (stats)
                (:if (mu:eq :t level-key)
                     ((:lambda (heap-info)
                        (core:%format
                         :t
                         "heap: type ~A npages ~A pagesize ~A~%"
                         `(,(mu:svref heap-info 0) ,(mu:svref heap-info 1) ,(mu:svref heap-info 2)))
                        (mu:fix
                         (:lambda (index)
                           (:if (core:null index)
                                ()
                                (:if (mu:less-than index (mu:vector-length stats))
                                     ((:lambda  ()
                                        ((:lambda (type size total free)
                                           (core:%format
                                            :t
                                            "      ~A total bytes: ~A in heap: ~A free: ~A~%"
                                            `(,(mu:symbol-name type) ,size ,total, free))
                                           (mu:add index 4))
                                         (mu:svref stats (mu:add 0 index))
                                         (mu:svref stats (mu:add 1 index))
                                         (mu:svref stats (mu:add 2 index))
                                         (mu:svref stats (mu:add 3 index)))
                                        (mu:add 4 index)))
                                     ())))
                         0))
                      (mu:heap-info))
                     (:if (mu:eq :null level-key)
                          (core:%prog2 (core:%format :t "~A~%" `(,stats)) ())
                          ((:lambda (heap-info)
                             (mu:fix
                              (:lambda (index)
                                (:if (core:null index)
                                     ()
                                     (:if (mu:less-than index (mu:vector-length stats))                                           
                                          ((:lambda (type size total free)
                                             (:if (mu:eq 0 size)
                                                  (mu:add 4 index)
                                                  (core:%format
                                                   :t
                                                   "type: ~A total bytes: ~A in heap: ~A free: ~A~%"
                                                   `(,type ,size ,total, free)))
                                             (mu:add index 4))
                                           (mu:svref stats (mu:add 0 index))
                                           (mu:svref stats (mu:add 1 index))
                                           (mu:svref stats (mu:add 2 index))
                                           (mu:svref stats (mu:add 3 index)))
                                          ())))
                              0))
                           (mu:heap-info)))))
              (mu:heap-stat))
             (core:raise verbosity 'common:room "not a room detail level")))
     (:if (core:null verbosity)
          :null
          verbosity))))
