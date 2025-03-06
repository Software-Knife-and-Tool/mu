;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; return the elapsed process time in usec
;;; from applying fn to args
;;;
(mu:intern (mu:find-namespace "") "perf" (mu:make-namespace "perf"))

(mu:intern perf "time-delta"
  (:lambda (fn args)
    ((:lambda (start stop)
       (mu:sub stop start))
       (cpu-time:process-time)
       ((:lambda ()
          (mu:apply fn args)
          (cpu-time:process-time))))))

;;;
;;; return an vector of storage consumed
;;; by applying fn to args
;;;
(mu:intern perf "storage-delta"
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
     (image:heap-stat)
     ((:lambda ()
         (mu:apply fn args)
         (image:heap-stat))))))
